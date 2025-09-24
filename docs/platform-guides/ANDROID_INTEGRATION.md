# 🤖 Android Integration Guide

*Complete guide to integrating Chess Engine Rust into your Android applications for mobile chess games and analysis tools.*

---

## 🚀 Quick Setup

### Gradle Configuration

```gradle
// app/build.gradle
android {
    compileSdk 34

    defaultConfig {
        applicationId "com.yourcompany.chessapp"
        minSdk 24  // Android 7.0+ for best performance
        targetSdk 34

        ndk {
            abiFilters 'arm64-v8a', 'armeabi-v7a', 'x86_64'  // Target architectures
        }
    }

    buildFeatures {
        dataBinding true
        viewBinding true
    }
}

dependencies {
    // Chess Engine Rust for Android
    implementation 'com.chess.engine:chess-engine-android:0.1.0'

    // UI and utility libraries
    implementation 'androidx.core:core-ktx:1.12.0'
    implementation 'androidx.appcompat:appcompat:1.6.1'
    implementation 'com.google.android.material:material:1.10.0'
    implementation 'androidx.constraintlayout:constraintlayout:2.1.4'
    implementation 'androidx.lifecycle:lifecycle-viewmodel-ktx:2.7.0'
    implementation 'org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3'
}
```

### Permissions

```xml
<!-- AndroidManifest.xml -->
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <!-- Optional: For saving games to external storage -->
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />

    <!-- Optional: For online features -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

    <application
        android:name=".ChessApplication"
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:theme="@style/AppTheme">

        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:screenOrientation="portrait">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>

    </application>
</manifest>
```

---

## 🎮 Basic Chess Game Implementation

### ChessEngine Wrapper (Kotlin)

```kotlin
// ChessEngineManager.kt
import com.chess.engine.ChessEngine
import com.chess.engine.EngineConfig
import kotlinx.coroutines.*
import android.util.Log

class ChessEngineManager(
    private val scope: CoroutineScope = CoroutineScope(Dispatchers.Default)
) {
    private var engine: ChessEngine? = null
    private val engineJob = SupervisorJob()

    // Engine configuration optimized for Android
    private val mobileConfig = EngineConfig.Builder()
        .setDepth(6)  // Balanced for mobile performance
        .setTimeLimitMs(3000)  // 3-second time limit
        .setThreads(2)  // Conservative threading for battery life
        .setHashSizeMB(32)  // 32MB hash table for mobile
        .setMobileOptimizations(true)  // Enable battery-friendly features
        .setBackgroundProcessing(false)  // Pause when app backgrounded
        .build()

    suspend fun initializeEngine(): Boolean = withContext(Dispatchers.Default) {
        try {
            engine = ChessEngine(mobileConfig)
            Log.i("ChessEngine", "Engine initialized successfully")
            true
        } catch (e: Exception) {
            Log.e("ChessEngine", "Failed to initialize engine", e)
            false
        }
    }

    suspend fun makeMove(moveStr: String): Result<String> = withContext(Dispatchers.Default) {
        try {
            engine?.makeMove(moveStr)
            Result.success("Move made: $moveStr")
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun findBestMove(): Result<String?> = withContext(Dispatchers.Default) {
        try {
            val bestMove = engine?.findBestMove()
            Result.success(bestMove)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    fun getCurrentPosition(): String? = engine?.getFen()

    fun getLegalMoves(): List<String> = engine?.getLegalMoves() ?: emptyList()

    fun getEvaluation(): Int = engine?.getEvaluation() ?: 0

    fun isGameOver(): Boolean = engine?.isGameOver() ?: false

    fun getGameResult(): String? = engine?.getGameResult()

    fun resetGame() {
        engine?.resetToStartingPosition()
    }

    fun cleanup() {
        engine?.cleanup()
        engine = null
        engineJob.cancel()
    }
}
```

### Chess Board View (Custom View)

```kotlin
// ChessBoardView.kt
import android.content.Context
import android.graphics.*
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.View
import androidx.core.content.ContextCompat

class ChessBoardView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {

    // Board state
    private val boardSize = 8
    private var squareSize = 0f
    private var selectedSquare: Pair<Int, Int>? = null
    private var legalMoves = emptyList<String>()

    // Position data
    private var piecePositions = mutableMapOf<String, PieceInfo>()

    // Paint objects
    private val lightSquarePaint = Paint().apply {
        color = ContextCompat.getColor(context, R.color.light_square)
        isAntiAlias = true
    }

    private val darkSquarePaint = Paint().apply {
        color = ContextCompat.getColor(context, R.color.dark_square)
        isAntiAlias = true
    }

    private val selectedSquarePaint = Paint().apply {
        color = ContextCompat.getColor(context, R.color.selected_square)
        isAntiAlias = true
    }

    private val legalMovePaint = Paint().apply {
        color = ContextCompat.getColor(context, R.color.legal_move_hint)
        isAntiAlias = true
        style = Paint.Style.FILL
    }

    private val piecePaint = Paint().apply {
        isAntiAlias = true
        textAlign = Paint.Align.CENTER
        typeface = Typeface.DEFAULT_BOLD
    }

    // Piece symbols (Unicode)
    private val pieceSymbols = mapOf(
        "white" to mapOf(
            "pawn" to "♙", "rook" to "♖", "knight" to "♘",
            "bishop" to "♗", "queen" to "♕", "king" to "♔"
        ),
        "black" to mapOf(
            "pawn" to "♟", "rook" to "♜", "knight" to "♞",
            "bishop" to "♝", "queen" to "♛", "king" to "♚"
        )
    )

    // Callbacks
    var onSquareClicked: ((file: Char, rank: Int) -> Unit)? = null
    var onMoveAttempted: ((fromSquare: String, toSquare: String) -> Unit)? = null

    override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
        super.onSizeChanged(w, h, oldw, oldh)
        val minDimension = minOf(w, h)
        squareSize = minDimension / boardSize.toFloat()
        piecePaint.textSize = squareSize * 0.8f
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)

        // Draw board squares
        for (rank in 0 until boardSize) {
            for (file in 0 until boardSize) {
                val isLightSquare = (rank + file) % 2 == 0
                val left = file * squareSize
                val top = rank * squareSize
                val right = left + squareSize
                val bottom = top + squareSize

                // Choose square color
                val paint = when {
                    selectedSquare == Pair(file, rank) -> selectedSquarePaint
                    isLightSquare -> lightSquarePaint
                    else -> darkSquarePaint
                }

                canvas.drawRect(left, top, right, bottom, paint)

                // Highlight legal move squares
                val squareName = "${('a' + file)}${8 - rank}"
                if (legalMoves.any { it.endsWith(squareName) }) {
                    canvas.drawCircle(
                        left + squareSize / 2,
                        top + squareSize / 2,
                        squareSize / 6,
                        legalMovePaint
                    )
                }
            }
        }

        // Draw pieces
        for ((square, piece) in piecePositions) {
            val file = square[0] - 'a'
            val rank = 8 - square[1].digitToInt()

            val symbol = pieceSymbols[piece.color]?.get(piece.type) ?: continue

            val x = (file + 0.5f) * squareSize
            val y = (rank + 0.5f) * squareSize

            // Draw piece shadow for better visibility
            piecePaint.color = Color.BLACK
            canvas.drawText(symbol, x + 2, y + 2, piecePaint)

            // Draw piece
            piecePaint.color = if (piece.color == "white") Color.WHITE else Color.BLACK
            canvas.drawText(symbol, x, y, piecePaint)
        }
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        if (event.action == MotionEvent.ACTION_DOWN) {
            val file = (event.x / squareSize).toInt()
            val rank = (event.y / squareSize).toInt()

            if (file in 0 until boardSize && rank in 0 until boardSize) {
                val clickedFile = ('a' + file)
                val clickedRank = 8 - rank
                val squareName = "$clickedFile$clickedRank"

                handleSquareClick(file, rank, squareName)
                return true
            }
        }
        return super.onTouchEvent(event)
    }

    private fun handleSquareClick(file: Int, rank: Int, squareName: String) {
        val currentSelected = selectedSquare

        if (currentSelected == null) {
            // First click - select square if it has a piece
            if (piecePositions.containsKey(squareName)) {
                selectedSquare = Pair(file, rank)
                onSquareClicked?.invoke(squareName[0], squareName[1].digitToInt())
            }
        } else {
            // Second click - attempt move or reselect
            val fromFile = ('a' + currentSelected.first)
            val fromRank = 8 - currentSelected.second
            val fromSquare = "$fromFile$fromRank"

            if (fromSquare == squareName) {
                // Clicked same square - deselect
                selectedSquare = null
            } else {
                // Attempt move
                onMoveAttempted?.invoke(fromSquare, squareName)
                selectedSquare = null
            }
        }

        invalidate() // Redraw
    }

    fun updatePosition(positionMap: Map<String, PieceInfo>) {
        piecePositions.clear()
        piecePositions.putAll(positionMap)
        invalidate()
    }

    fun setLegalMoves(moves: List<String>) {
        legalMoves = moves
        invalidate()
    }

    fun clearSelection() {
        selectedSquare = null
        invalidate()
    }
}

data class PieceInfo(
    val type: String,
    val color: String
)
```

### Main Game Activity

```kotlin
// MainActivity.kt
import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.yourcompany.chessapp.databinding.ActivityMainBinding
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding
    private lateinit var chessEngine: ChessEngineManager

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setupChessEngine()
        setupUI()
    }

    private fun setupChessEngine() {
        chessEngine = ChessEngineManager(lifecycleScope)

        lifecycleScope.launch {
            binding.progressBar.visibility = android.view.View.VISIBLE
            binding.statusText.text = "Initializing chess engine..."

            val success = chessEngine.initializeEngine()

            binding.progressBar.visibility = android.view.View.GONE

            if (success) {
                binding.statusText.text = "Ready to play!"
                updateBoardDisplay()
            } else {
                binding.statusText.text = "Failed to initialize engine"
                Toast.makeText(this@MainActivity,
                    "Chess engine initialization failed",
                    Toast.LENGTH_LONG).show()
            }
        }
    }

    private fun setupUI() {
        // Chess board interactions
        binding.chessBoardView.onMoveAttempted = { fromSquare, toSquare ->
            makeMove("$fromSquare$toSquare")
        }

        binding.chessBoardView.onSquareClicked = { file, rank ->
            val squareName = "$file$rank"
            val legalMoves = chessEngine.getLegalMoves()
            val movesFromSquare = legalMoves.filter { it.startsWith(squareName) }
            binding.chessBoardView.setLegalMoves(movesFromSquare)
        }

        // Control buttons
        binding.newGameButton.setOnClickListener { startNewGame() }
        binding.undoButton.setOnClickListener { undoMove() }
        binding.hintButton.setOnClickListener { showHint() }

        // Engine move button
        binding.engineMoveButton.setOnClickListener {
            lifecycleScope.launch {
                makeEngineMove()
            }
        }

        // Difficulty selector
        binding.difficultySpinner.setSelection(1) // Medium difficulty
    }

    private fun makeMove(moveStr: String) {
        lifecycleScope.launch {
            val result = chessEngine.makeMove(moveStr)

            result.onSuccess { message ->
                binding.statusText.text = message
                updateBoardDisplay()

                // Check game state
                if (chessEngine.isGameOver()) {
                    handleGameOver()
                } else {
                    // Auto-play engine response after human move
                    makeEngineMove()
                }
            }

            result.onFailure { exception ->
                Toast.makeText(this@MainActivity,
                    "Invalid move: ${exception.message}",
                    Toast.LENGTH_SHORT).show()
                binding.chessBoardView.clearSelection()
            }
        }
    }

    private suspend fun makeEngineMove() {
        binding.progressBar.visibility = android.view.View.VISIBLE
        binding.statusText.text = "Engine is thinking..."
        binding.engineMoveButton.isEnabled = false

        try {
            val result = chessEngine.findBestMove()

            result.onSuccess { bestMove ->
                if (bestMove != null) {
                    chessEngine.makeMove(bestMove)
                    val evaluation = chessEngine.getEvaluation()

                    binding.statusText.text = "Engine played: $bestMove (${evaluation} cp)"
                    updateBoardDisplay()

                    if (chessEngine.isGameOver()) {
                        handleGameOver()
                    }
                } else {
                    binding.statusText.text = "No legal moves available"
                }
            }

            result.onFailure { exception ->
                Toast.makeText(this,
                    "Engine error: ${exception.message}",
                    Toast.LENGTH_SHORT).show()
            }

        } finally {
            binding.progressBar.visibility = android.view.View.GONE
            binding.engineMoveButton.isEnabled = true
        }
    }

    private fun updateBoardDisplay() {
        // Get current position from engine
        val currentFen = chessEngine.getCurrentPosition()
        if (currentFen != null) {
            val positionMap = parseFenToPositionMap(currentFen)
            binding.chessBoardView.updatePosition(positionMap)
        }

        // Update game info
        val evaluation = chessEngine.getEvaluation()
        val legalMovesCount = chessEngine.getLegalMoves().size

        binding.evaluationText.text = "Evaluation: ${evaluation} cp"
        binding.movesCountText.text = "Legal moves: $legalMovesCount"

        // Update evaluation bar
        updateEvaluationBar(evaluation)
    }

    private fun parseFenToPositionMap(fen: String): Map<String, PieceInfo> {
        val positionMap = mutableMapOf<String, PieceInfo>()
        val fenParts = fen.split(" ")
        val boardState = fenParts[0]

        var rank = 8
        var file = 0

        for (char in boardState) {
            when {
                char == '/' -> {
                    rank--
                    file = 0
                }
                char.isDigit() -> {
                    file += char.digitToInt()
                }
                else -> {
                    val isWhite = char.isUpperCase()
                    val pieceType = when (char.lowercaseChar()) {
                        'p' -> "pawn"
                        'r' -> "rook"
                        'n' -> "knight"
                        'b' -> "bishop"
                        'q' -> "queen"
                        'k' -> "king"
                        else -> continue
                    }

                    val squareName = "${('a' + file)}$rank"
                    positionMap[squareName] = PieceInfo(
                        type = pieceType,
                        color = if (isWhite) "white" else "black"
                    )
                    file++
                }
            }
        }

        return positionMap
    }

    private fun updateEvaluationBar(evaluation: Int) {
        // Update evaluation bar (range: -500 to +500 centipawns)
        val normalizedEval = evaluation.coerceIn(-500, 500)
        val percentage = ((normalizedEval + 500) / 1000f * 100).toInt()

        binding.evaluationBar.progress = percentage

        // Color the bar based on advantage
        val color = when {
            evaluation > 100 -> ContextCompat.getColor(this, R.color.white_advantage)
            evaluation < -100 -> ContextCompat.getColor(this, R.color.black_advantage)
            else -> ContextCompat.getColor(this, R.color.equal_position)
        }

        binding.evaluationBar.progressTintList = ColorStateList.valueOf(color)
    }

    private fun startNewGame() {
        chessEngine.resetGame()
        updateBoardDisplay()
        binding.statusText.text = "New game started!"
        binding.chessBoardView.clearSelection()
    }

    private fun undoMove() {
        // Implementation depends on engine undo capability
        Toast.makeText(this, "Undo functionality coming soon!", Toast.LENGTH_SHORT).show()
    }

    private fun showHint() {
        lifecycleScope.launch {
            binding.progressBar.visibility = android.view.View.VISIBLE

            val result = chessEngine.findBestMove()

            result.onSuccess { bestMove ->
                if (bestMove != null) {
                    Toast.makeText(this@MainActivity,
                        "Hint: Consider playing $bestMove",
                        Toast.LENGTH_LONG).show()
                } else {
                    Toast.makeText(this@MainActivity,
                        "No moves available",
                        Toast.LENGTH_SHORT).show()
                }
            }

            binding.progressBar.visibility = android.view.View.GONE
        }
    }

    private fun handleGameOver() {
        val result = chessEngine.getGameResult()
        binding.statusText.text = "Game Over! Result: $result"

        // Show game over dialog
        androidx.appcompat.app.AlertDialog.Builder(this)
            .setTitle("Game Over")
            .setMessage("Result: $result")
            .setPositiveButton("New Game") { _, _ -> startNewGame() }
            .setNegativeButton("Close", null)
            .show()
    }

    override fun onDestroy() {
        super.onDestroy()
        chessEngine.cleanup()
    }
}
```

---

## 📱 Advanced Features

### Game Analysis Activity

```kotlin
// AnalysisActivity.kt
class AnalysisActivity : AppCompatActivity() {

    private lateinit var binding: ActivityAnalysisBinding
    private lateinit var chessEngine: ChessEngineManager
    private val gameHistory = mutableListOf<GameMove>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityAnalysisBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setupAnalysisEngine()
        setupCharts()
    }

    private fun setupAnalysisEngine() {
        // High-depth engine for analysis
        val analysisConfig = EngineConfig.Builder()
            .setDepth(12)  // Deep analysis
            .setTimeLimitMs(10000)  // 10 seconds per position
            .setThreads(4)  // Use all available cores
            .setHashSizeMB(128)  // Larger hash table
            .build()

        chessEngine = ChessEngineManager(lifecycleScope)
        // Initialize with analysis config...
    }

    fun analyzeGame(pgnMoves: List<String>) {
        lifecycleScope.launch {
            binding.analysisProgress.visibility = View.VISIBLE
            binding.analysisStatus.text = "Analyzing game..."

            val analysisResults = mutableListOf<MoveAnalysis>()

            for ((moveNum, move) in pgnMoves.withIndex()) {
                // Update progress
                val progress = (moveNum * 100) / pgnMoves.size
                binding.analysisProgress.progress = progress
                binding.analysisStatus.text = "Analyzing move ${moveNum + 1}/${pgnMoves.size}"

                // Make the move
                chessEngine.makeMove(move)

                // Analyze the resulting position
                val evaluation = chessEngine.getEvaluation()
                val bestMove = chessEngine.findBestMove().getOrNull()

                val analysis = MoveAnalysis(
                    moveNumber = moveNum + 1,
                    movePlayed = move,
                    evaluation = evaluation,
                    bestMove = bestMove,
                    isBlunder = false, // Calculate based on evaluation swing
                    centipawnLoss = 0 // Calculate based on previous evaluation
                )

                analysisResults.add(analysis)
            }

            // Process results and update UI
            displayAnalysisResults(analysisResults)

            binding.analysisProgress.visibility = View.GONE
            binding.analysisStatus.text = "Analysis complete!"
        }
    }

    private fun displayAnalysisResults(results: List<MoveAnalysis>) {
        // Create evaluation chart
        createEvaluationChart(results)

        // Update statistics
        updateAnalysisStatistics(results)

        // Populate move list with annotations
        populateMoveList(results)
    }

    private fun createEvaluationChart(results: List<MoveAnalysis>) {
        val chart = binding.evaluationChart

        val entries = results.mapIndexed { index, analysis ->
            Entry(index.toFloat(), analysis.evaluation.toFloat())
        }

        val dataSet = LineDataSet(entries, "Position Evaluation").apply {
            color = ContextCompat.getColor(this@AnalysisActivity, R.color.eval_line)
            lineWidth = 2f
            setDrawValues(false)
            setDrawCircles(false)
        }

        val lineData = LineData(dataSet)
        chart.data = lineData

        // Customize chart
        chart.description.isEnabled = false
        chart.legend.isEnabled = true
        chart.setTouchEnabled(true)
        chart.isDragEnabled = true
        chart.setScaleEnabled(true)

        // Add zero line for equal evaluation
        val zeroLine = LimitLine(0f, "Equal")
        chart.axisLeft.addLimitLine(zeroLine)

        chart.invalidate() // Refresh chart
    }
}

data class GameMove(
    val moveNumber: Int,
    val move: String,
    val fen: String,
    val evaluation: Int,
    val timeSpent: Long
)

data class MoveAnalysis(
    val moveNumber: Int,
    val movePlayed: String,
    val evaluation: Int,
    val bestMove: String?,
    val isBlunder: Boolean,
    val centipawnLoss: Int
)
```

### Settings and Preferences

```kotlin
// SettingsActivity.kt
class SettingsActivity : PreferenceFragmentCompat() {

    override fun onCreatePreferences(savedInstanceState: Bundle?, rootKey: String?) {
        setPreferencesFromResource(R.xml.preferences, rootKey)

        // Engine strength setting
        findPreference<SeekBarPreference>("engine_depth")?.apply {
            min = 1
            max = 15
            setDefaultValue(6)
            summary = "Current: %s ply"

            setOnPreferenceChangeListener { _, newValue ->
                updateEngineDifficulty(newValue as Int)
                true
            }
        }

        // Time control setting
        findPreference<SeekBarPreference>("time_control")?.apply {
            min = 1
            max = 30
            setDefaultValue(5)
            summary = "Current: %s seconds"
        }

        // Board theme setting
        findPreference<ListPreference>("board_theme")?.apply {
            entries = arrayOf("Classic", "Wood", "Marble", "Neon")
            entryValues = arrayOf("classic", "wood", "marble", "neon")
            setDefaultValue("classic")

            setOnPreferenceChangeListener { _, newValue ->
                updateBoardTheme(newValue as String)
                true
            }
        }

        // Sound effects
        findPreference<SwitchPreferenceCompat>("sound_effects")?.apply {
            setDefaultValue(true)
        }

        // Vibration on moves
        findPreference<SwitchPreferenceCompat>("vibration")?.apply {
            setDefaultValue(true)
        }

        // Analysis features
        findPreference<SwitchPreferenceCompat>("show_coordinates")?.apply {
            setDefaultValue(false)
        }

        findPreference<SwitchPreferenceCompat>("highlight_last_move")?.apply {
            setDefaultValue(true)
        }
    }

    private fun updateEngineDifficulty(depth: Int) {
        // Update engine configuration
        val sharedPrefs = PreferenceManager.getDefaultSharedPreferences(requireContext())
        sharedPrefs.edit().putInt("engine_depth", depth).apply()

        // Notify main activity to update engine
        requireContext().sendBroadcast(Intent("UPDATE_ENGINE_CONFIG"))
    }

    private fun updateBoardTheme(theme: String) {
        val sharedPrefs = PreferenceManager.getDefaultSharedPreferences(requireContext())
        sharedPrefs.edit().putString("board_theme", theme).apply()

        // Notify to update UI theme
        requireContext().sendBroadcast(Intent("UPDATE_BOARD_THEME"))
    }
}
```

---

## 🎨 UI and UX Enhancements

### Material Design Layout

```xml
<!-- activity_main.xml -->
<?xml version="1.0" encoding="utf-8"?>
<androidx.coordinatorlayout.widget.CoordinatorLayout
    xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:app="http://schemas.android.com/apk/res-auto"
    android:layout_width="match_parent"
    android:layout_height="match_parent">

    <!-- App Bar -->
    <com.google.android.material.appbar.AppBarLayout
        android:layout_width="match_parent"
        android:layout_height="wrap_content">

        <com.google.android.material.appbar.MaterialToolbar
            android:id="@+id/toolbar"
            android:layout_width="match_parent"
            android:layout_height="?attr/actionBarSize"
            android:background="?attr/colorPrimary"
            app:title="Chess Master"
            app:titleTextColor="?attr/colorOnPrimary" />

    </com.google.android.material.appbar.AppBarLayout>

    <!-- Main Content -->
    <androidx.core.widget.NestedScrollView
        android:layout_width="match_parent"
        android:layout_height="match_parent"
        app:layout_behavior="@string/appbar_scrolling_view_behavior">

        <LinearLayout
            android:layout_width="match_parent"
            android:layout_height="wrap_content"
            android:orientation="vertical"
            android:padding="16dp">

            <!-- Game Status Card -->
            <com.google.android.material.card.MaterialCardView
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                android:layout_marginBottom="16dp"
                app:cardCornerRadius="12dp"
                app:cardElevation="4dp">

                <LinearLayout
                    android:layout_width="match_parent"
                    android:layout_height="wrap_content"
                    android:orientation="vertical"
                    android:padding="16dp">

                    <TextView
                        android:id="@+id/statusText"
                        android:layout_width="match_parent"
                        android:layout_height="wrap_content"
                        android:text="Initializing..."
                        android:textAppearance="?attr/textAppearanceHeadline6"
                        android:gravity="center" />

                    <ProgressBar
                        android:id="@+id/progressBar"
                        style="?android:attr/progressBarStyleHorizontal"
                        android:layout_width="match_parent"
                        android:layout_height="wrap_content"
                        android:layout_marginTop="8dp"
                        android:visibility="gone" />

                </LinearLayout>

            </com.google.android.material.card.MaterialCardView>

            <!-- Chess Board Card -->
            <com.google.android.material.card.MaterialCardView
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                android:layout_marginBottom="16dp"
                app:cardCornerRadius="12dp"
                app:cardElevation="4dp">

                <com.yourcompany.chessapp.ChessBoardView
                    android:id="@+id/chessBoardView"
                    android:layout_width="match_parent"
                    android:layout_height="400dp"
                    android:layout_margin="8dp" />

            </com.google.android.material.card.MaterialCardView>

            <!-- Evaluation Card -->
            <com.google.android.material.card.MaterialCardView
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                android:layout_marginBottom="16dp"
                app:cardCornerRadius="12dp"
                app:cardElevation="4dp">

                <LinearLayout
                    android:layout_width="match_parent"
                    android:layout_height="wrap_content"
                    android:orientation="vertical"
                    android:padding="16dp">

                    <TextView
                        android:layout_width="match_parent"
                        android:layout_height="wrap_content"
                        android:text="Position Evaluation"
                        android:textAppearance="?attr/textAppearanceSubtitle1"
                        android:textStyle="bold" />

                    <ProgressBar
                        android:id="@+id/evaluationBar"
                        style="?android:attr/progressBarStyleHorizontal"
                        android:layout_width="match_parent"
                        android:layout_height="wrap_content"
                        android:layout_marginTop="8dp"
                        android:max="100"
                        android:progress="50" />

                    <LinearLayout
                        android:layout_width="match_parent"
                        android:layout_height="wrap_content"
                        android:orientation="horizontal"
                        android:layout_marginTop="8dp">

                        <TextView
                            android:id="@+id/evaluationText"
                            android:layout_width="0dp"
                            android:layout_height="wrap_content"
                            android:layout_weight="1"
                            android:text="Evaluation: 0 cp"
                            android:textAppearance="?attr/textAppearanceBody2" />

                        <TextView
                            android:id="@+id/movesCountText"
                            android:layout_width="wrap_content"
                            android:layout_height="wrap_content"
                            android:text="Legal moves: 20"
                            android:textAppearance="?attr/textAppearanceBody2" />

                    </LinearLayout>

                </LinearLayout>

            </com.google.android.material.card.MaterialCardView>

            <!-- Control Buttons -->
            <LinearLayout
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                android:orientation="horizontal"
                android:gravity="center">

                <com.google.android.material.button.MaterialButton
                    android:id="@+id/newGameButton"
                    style="@style/Widget.Material3.Button.OutlinedButton"
                    android:layout_width="0dp"
                    android:layout_height="wrap_content"
                    android:layout_weight="1"
                    android:layout_marginEnd="4dp"
                    android:text="New Game"
                    app:icon="@drawable/ic_refresh" />

                <com.google.android.material.button.MaterialButton
                    android:id="@+id/undoButton"
                    style="@style/Widget.Material3.Button.OutlinedButton"
                    android:layout_width="0dp"
                    android:layout_height="wrap_content"
                    android:layout_weight="1"
                    android:layout_marginStart="4dp"
                    android:layout_marginEnd="4dp"
                    android:text="Undo"
                    app:icon="@drawable/ic_undo" />

                <com.google.android.material.button.MaterialButton
                    android:id="@+id/hintButton"
                    style="@style/Widget.Material3.Button.OutlinedButton"
                    android:layout_width="0dp"
                    android:layout_height="wrap_content"
                    android:layout_weight="1"
                    android:layout_marginStart="4dp"
                    android:text="Hint"
                    app:icon="@drawable/ic_lightbulb" />

            </LinearLayout>

            <!-- Engine Move Button -->
            <com.google.android.material.button.MaterialButton
                android:id="@+id/engineMoveButton"
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                android:layout_marginTop="16dp"
                android:text="Engine Move"
                app:icon="@drawable/ic_smart_toy" />

        </LinearLayout>

    </androidx.core.widget.NestedScrollView>

    <!-- Floating Action Button for Analysis -->
    <com.google.android.material.floatingactionbutton.FloatingActionButton
        android:id="@+id/analysisFab"
        android:layout_width="wrap_content"
        android:layout_height="wrap_content"
        android:layout_margin="16dp"
        android:layout_gravity="bottom|end"
        app:srcCompat="@drawable/ic_analytics"
        app:tint="?attr/colorOnSecondary" />

</androidx.coordinatorlayout.widget.CoordinatorLayout>
```

---

## ⚡ Performance Optimization

### Battery-Friendly Configuration

```kotlin
// PowerManagementHelper.kt
class PowerManagementHelper(private val context: Context) {

    private val powerManager = context.getSystemService(Context.POWER_SERVICE) as PowerManager

    fun getOptimalEngineConfig(): EngineConfig {
        val batteryLevel = getBatteryLevel()
        val isCharging = isCharging()
        val isPowerSaveMode = powerManager.isPowerSaveMode

        return when {
            isPowerSaveMode || batteryLevel < 20 -> getLowPowerConfig()
            batteryLevel < 50 && !isCharging -> getBalancedConfig()
            else -> getPerformanceConfig()
        }
    }

    private fun getLowPowerConfig() = EngineConfig.Builder()
        .setDepth(4)  // Shallow search
        .setTimeLimitMs(1500)  // Quick moves
        .setThreads(1)  // Single threaded
        .setHashSizeMB(16)  // Small hash table
        .setMobileOptimizations(true)
        .setLowPowerMode(true)
        .build()

    private fun getBalancedConfig() = EngineConfig.Builder()
        .setDepth(6)  // Moderate search
        .setTimeLimitMs(3000)  // 3 second limit
        .setThreads(2)  // Limited threading
        .setHashSizeMB(32)  // Moderate hash
        .setMobileOptimizations(true)
        .build()

    private fun getPerformanceConfig() = EngineConfig.Builder()
        .setDepth(8)  // Deep search
        .setTimeLimitMs(5000)  // 5 second limit
        .setThreads(4)  // Full threading
        .setHashSizeMB(64)  // Large hash table
        .setMobileOptimizations(false)  // Full features
        .build()

    private fun getBatteryLevel(): Int {
        val batteryManager = context.getSystemService(Context.BATTERY_SERVICE) as BatteryManager
        return batteryManager.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY)
    }

    private fun isCharging(): Boolean {
        val intentFilter = IntentFilter(Intent.ACTION_BATTERY_CHANGED)
        val batteryStatus = context.registerReceiver(null, intentFilter)
        val status = batteryStatus?.getIntExtra(BatteryManager.EXTRA_STATUS, -1) ?: -1

        return status == BatteryManager.BATTERY_STATUS_CHARGING ||
               status == BatteryManager.BATTERY_STATUS_FULL
    }
}
```

### Memory Management

```kotlin
// MemoryManager.kt
class MemoryManager(private val context: Context) {

    private val activityManager = context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager

    fun optimizeForDevice(): EngineConfig {
        val memoryInfo = ActivityManager.MemoryInfo()
        activityManager.getMemoryInfo(memoryInfo)

        val availableMemoryMB = (memoryInfo.availMem / (1024 * 1024)).toInt()
        val isLowMemoryDevice = activityManager.isLowRamDevice

        val hashSizeMB = when {
            isLowMemoryDevice -> 16
            availableMemoryMB < 1024 -> 32  // Less than 1GB available
            availableMemoryMB < 2048 -> 64  // Less than 2GB available
            else -> 128  // Plenty of memory
        }

        return EngineConfig.Builder()
            .setHashSizeMB(hashSizeMB)
            .setPreallocateMemory(!isLowMemoryDevice)
            .setGarbageCollectionOptimized(isLowMemoryDevice)
            .build()
    }

    fun monitorMemoryUsage() {
        // Monitor memory usage and adjust engine parameters
        val runtime = Runtime.getRuntime()
        val maxMemory = runtime.maxMemory()
        val totalMemory = runtime.totalMemory()
        val freeMemory = runtime.freeMemory()
        val usedMemory = totalMemory - freeMemory

        val memoryUsagePercent = (usedMemory * 100) / maxMemory

        if (memoryUsagePercent > 80) {
            // High memory usage - trigger cleanup
            System.gc()  // Suggest garbage collection
            // Reduce engine cache sizes if possible
        }
    }
}
```

---

## 📦 Distribution and Deployment

### ProGuard Configuration

```pro
# proguard-rules.pro

# Keep chess engine classes
-keep class com.chess.engine.** { *; }

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep model classes
-keep class com.yourcompany.chessapp.model.** { *; }

# Keep custom views
-keep class com.yourcompany.chessapp.ChessBoardView { *; }

# Optimize but don't obfuscate chess logic
-keep class com.yourcompany.chessapp.ChessEngineManager { *; }

# Remove logging in release builds
-assumenosideeffects class android.util.Log {
    public static boolean isLoggable(java.lang.String, int);
    public static int v(...);
    public static int i(...);
    public static int w(...);
    public static int d(...);
    public static int e(...);
}
```

### Gradle Build Configuration

```gradle
// app/build.gradle
android {
    compileSdk 34

    defaultConfig {
        applicationId "com.yourcompany.chessapp"
        minSdk 24
        targetSdk 34
        versionCode 1
        versionName "1.0.0"

        // Enable multidex for large applications
        multiDexEnabled true

        // Native library configuration
        ndk {
            abiFilters 'arm64-v8a', 'armeabi-v7a', 'x86_64'
        }
    }

    buildTypes {
        debug {
            debuggable true
            minifyEnabled false
            applicationIdSuffix ".debug"
        }

        release {
            debuggable false
            minifyEnabled true
            shrinkResources true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'),
                         'proguard-rules.pro'

            // Signing configuration
            signingConfig signingConfigs.release
        }
    }

    // Enable data binding and view binding
    buildFeatures {
        dataBinding true
        viewBinding true
    }

    // Compile options
    compileOptions {
        sourceCompatibility JavaVersion.VERSION_17
        targetCompatibility JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = '17'
    }
}

// Dependencies optimization
dependencies {
    implementation 'com.chess.engine:chess-engine-android:0.1.0'

    // Core Android libraries
    implementation 'androidx.core:core-ktx:1.12.0'
    implementation 'androidx.appcompat:appcompat:1.6.1'
    implementation 'androidx.constraintlayout:constraintlayout:2.1.4'
    implementation 'androidx.lifecycle:lifecycle-viewmodel-ktx:2.7.0'

    // Material Design
    implementation 'com.google.android.material:material:1.10.0'

    // Coroutines
    implementation 'org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3'

    // Charts (for analysis features)
    implementation 'com.github.PhilJay:MPAndroidChart:v3.1.0'

    // Preferences
    implementation 'androidx.preference:preference-ktx:1.2.1'
}
```

---

## 🧪 Testing

### Unit Tests

```kotlin
// ChessEngineManagerTest.kt
@RunWith(AndroidJUnit4::class)
class ChessEngineManagerTest {

    private lateinit var chessEngine: ChessEngineManager

    @Before
    fun setup() {
        chessEngine = ChessEngineManager()
        runBlocking {
            chessEngine.initializeEngine()
        }
    }

    @Test
    fun testInitialPosition() {
        val legalMoves = chessEngine.getLegalMoves()
        assertEquals(20, legalMoves.size) // 20 legal moves from starting position
    }

    @Test
    fun testMakeValidMove() = runBlocking {
        val result = chessEngine.makeMove("e2e4")
        assertTrue(result.isSuccess)
    }

    @Test
    fun testMakeInvalidMove() = runBlocking {
        val result = chessEngine.makeMove("invalid_move")
        assertTrue(result.isFailure)
    }

    @Test
    fun testEvaluationRange() {
        val evaluation = chessEngine.getEvaluation()
        assertTrue(evaluation in -10000..10000) // Reasonable evaluation range
    }

    @After
    fun cleanup() {
        chessEngine.cleanup()
    }
}
```

### UI Tests

```kotlin
// MainActivityTest.kt
@RunWith(AndroidJUnit4::class)
class MainActivityTest {

    @get:Rule
    val activityRule = ActivityScenarioRule(MainActivity::class.java)

    @Test
    fun testChessBoardDisplayed() {
        onView(withId(R.id.chessBoardView))
            .check(matches(isDisplayed()))
    }

    @Test
    fun testNewGameButton() {
        onView(withId(R.id.newGameButton))
            .perform(click())

        onView(withText("New game started!"))
            .check(matches(isDisplayed()))
    }

    @Test
    fun testEngineMove() {
        // Wait for engine initialization
        Thread.sleep(2000)

        onView(withId(R.id.engineMoveButton))
            .perform(click())

        // Wait for engine move
        Thread.sleep(5000)

        onView(withText(containsString("Engine played:")))
            .check(matches(isDisplayed()))
    }
}
```

---

## 🚀 Deployment

### Google Play Store

```kotlin
// Play Store listing optimization
android {
    bundle {
        language {
            enableSplit = true
        }
        density {
            enableSplit = true
        }
        abi {
            enableSplit = true
        }
    }
}
```

### Release Checklist

- [ ] ✅ Test on multiple device sizes and orientations
- [ ] ⚡ Verify performance on low-end devices
- [ ] 🔋 Test battery consumption under various conditions
- [ ] 📱 Ensure proper behavior during interruptions (calls, notifications)
- [ ] 💾 Test with limited storage space
- [ ] 🌐 Verify offline functionality
- [ ] 🎨 Test with different themes and accessibility settings
- [ ] 📊 Analytics and crash reporting configured
- [ ] 🔒 Security review completed
- [ ] 📝 Store listing and screenshots prepared

---

## 📚 Resources

- **📖 Android Chess Engine API**: Refer to the JNI documentation and Android examples in this repository
- **🎨 Material Design**: [Material Design Guidelines](https://material.io/design)
- **⚡ Android Performance**: [Android Performance Best Practices](https://developer.android.com/topic/performance)
- **🔋 Battery Optimization**: [Android Battery Optimization](https://developer.android.com/topic/performance/power)

---

*Build amazing chess experiences on Android! 🤖♟️*