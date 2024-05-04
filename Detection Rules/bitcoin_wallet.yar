rule Detect_Bitcoin_Wallet_1f3YIXo6YTbweBeO61CWOyHkQzH8ub2fHZ {
    meta:
        description = "Detects the Bitcoin Wallet address '1f3YIXo6YTbweBeO61CWOyHkQzH8ub2fHZ'"
    strings:
        $wallet_address = "1f3YIXo6YTbweBeO61CWOyHkQzH8ub2fHZ"
    condition:
        $wallet_address
}
