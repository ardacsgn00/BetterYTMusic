package com.betterytmusic.app

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    try {
      enableEdgeToEdge()
    } catch (_: Throwable) {}
    super.onCreate(savedInstanceState)
  }
}
