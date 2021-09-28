package taweel.ameer

import io.ktor.server.engine.*
import io.ktor.server.netty.*
import taweel.ameer.plugins.configureRouting

fun main() {
    embeddedServer(Netty, port = 6969, host = "0.0.0.0") {
        configureRouting()
    }.start(wait = true)
}
