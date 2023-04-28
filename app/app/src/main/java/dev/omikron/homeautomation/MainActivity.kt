package dev.omikron.homeautomation

import android.annotation.SuppressLint
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.webkit.WebView
import android.webkit.WebViewClient

class MainActivity : AppCompatActivity() {

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val webView = findViewById<WebView>(R.id.web)
        webView.loadUrl("http://10.0.2.2:8080")
        webView.settings.javaScriptEnabled = true
        webView.settings.userAgentString = "Home Automation Client"
        webView.webViewClient = WebViewClient()
    }
}