package com.open_witness_library.app

import android.os.Bundle
import android.view.View
import android.view.WindowManager
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.updatePadding

class MainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        WindowCompat.setDecorFitsSystemWindows(window, false)

        // Handling window insets manually
        findViewById<View>(android.R.id.content).setOnApplyWindowInsetsListener { view, insets ->
            val imeInsets = insets.getInsets(WindowInsetsCompat.Type.ime())
            view.updatePadding(bottom = imeInsets.bottom)
            insets
        }
    }
}