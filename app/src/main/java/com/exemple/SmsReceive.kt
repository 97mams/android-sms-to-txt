class SmsReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context, intent: Intent) {
        val bundle = intent.extras ?: return
        val pdus = bundle["pdus"] as? Array<*> ?: return

        for (pdu in pdus) {
            val sms = SmsMessage.createFromPdu(pdu as ByteArray)

            val message =
                "${sms.timestampMillis} | ${sms.originatingAddress}: ${sms.messageBody}\n"

            SmsWriter.append(context, message)
        }
    }
}
