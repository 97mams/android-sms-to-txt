object SmsWriter {

    fun append(context: Context, text: String) {
        try {
            val file = File(context.filesDir, "sms.txt")
            file.appendText(text)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}
