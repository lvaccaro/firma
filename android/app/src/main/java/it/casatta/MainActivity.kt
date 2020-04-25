package it.casatta

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        title = "Firma (${Network.TYPE})"

        key_button.setOnClickListener {
            ListActivity.comeHere(this, ListActivity.KEYS)
        }

        wallet_button.setOnClickListener {
            ListActivity.comeHere(this, ListActivity.WALLETS)
        }

        psbt_button.setOnClickListener {
            ListActivity.comeHere(this, ListActivity.PSBTS)
        }

        sign_button.isEnabled = true
        sign_button.setOnClickListener {
            if (key_text.text == getString(R.string.select_key) || wallet_text.text == getString(R.string.select_wallet)  || psbt_text.text == getString(R.string.select_psbt)) {
                Toast.makeText(this, "Select key, wallet and psbt", Toast.LENGTH_LONG).show()
            } else {
                val keyFile = "$filesDir/${Network.TYPE}/keys/${key_text.text}/PRIVATE.json"
                val walletFile = "$filesDir/${Network.TYPE}/wallets/${wallet_text.text}/descriptor.json"
                val psbtFile = "$filesDir/${Network.TYPE}/psbts/${psbt_text.text}/psbt.json"
                val result = Rust().sign(filesDir.toString(), keyFile,walletFile, psbtFile )
                AlertDialog.Builder(this).setMessage(result.toString()).create().show()
            }
        }
    }

    override fun onActivityResult(
        requestCode: Int,
        resultCode: Int,
        data: Intent?
    ) {
        if (resultCode == Activity.RESULT_OK) {
            val result = data?.getStringExtra(C.RESULT)
            if (requestCode == ListActivity.KEYS) {
                key_text.text = result
            } else if (requestCode == ListActivity.WALLETS) {
                wallet_text.text = result
            } else if (requestCode == ListActivity.PSBTS) {
                psbt_text.text = result
            }
        } else {
            super.onActivityResult(requestCode, resultCode, data)
        }
    }
}

fun ByteArray.toHexString() = joinToString("") { "%02x".format(it) }