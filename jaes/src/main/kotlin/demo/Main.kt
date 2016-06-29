package demo

import org.apache.shiro.crypto.AesCipherService
import java.util.*

fun main(args: Array<String>) {
    val aes = AesCipherService()

    println("new client id: ${UUID.randomUUID().toString().replace("-", "")}")
    println("new client secret: ${Base64.getEncoder().encodeToString(aes.generateNewKey().encoded)}")
}
