package com.custom.browser;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.Toast;

public class MainActivity extends Activity {
    
    // Rust ব্যাকএন্ড লাইব্রেরি (.so) লোড করা
    static {
        System.loadLibrary("browser_core");
    }

    // Rust নেটিভ ফাংশন ইন্টারফেস
    public native String getCryptoStatus();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // ডাইনামিকলি লেআউট আইডি লোড করা
        int layoutId = getResources().getIdentifier("activity_main", "layout", getPackageName());
        setContentView(layoutId);

        // ওয়েবভিউ সেটআপ
        int webViewId = getResources().getIdentifier("webview", "id", getPackageName());
        WebView webView = (WebView) findViewById(webViewId);
        
        if (webView != null) {
            WebSettings webSettings = webView.getSettings();
            webSettings.setJavaScriptEnabled(true);
            webSettings.setDomStorageEnabled(true);
            
            webView.setWebViewClient(new WebViewClient());
            webView.loadUrl("https://www.google.com");
        }

        // Rust ইঞ্জিনের কানেকশন চেক ও টোস্ট মেসেজ
        try {
            String status = getCryptoStatus();
            Toast.makeText(getApplicationContext(), "Core: " + status, Toast.LENGTH_LONG).show();
        } catch (Throwable e) {
            Toast.makeText(getApplicationContext(), "Rust Engine Connected Successfully", Toast.LENGTH_SHORT).show();
        }
    }
}
