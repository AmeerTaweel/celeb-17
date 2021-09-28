package taweel.ameer.plugins

import io.ktor.application.*
import io.ktor.http.*
import io.ktor.response.*
import io.ktor.routing.*
import taweel.ameer.managers.ConfigManager
import taweel.ameer.managers.MusicPlayer
import taweel.ameer.utils.SongGenerator
import taweel.ameer.utils.SongProvider

fun Application.configureRouting() {
    routing {
        get("/play/all") {
            val allSongsGenerator = SongGenerator(ConfigManager.getAllSongs())
            SongProvider.setGeneratorAndRegenerateQuery(allSongsGenerator)
            MusicPlayer.startAndNextIfStarted()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/play/tag/{tag}") {
            val tag = call.parameters["tag"]!!
            val songsByTagGenerator = SongGenerator(ConfigManager.getSongsByTag(tag))
            SongProvider.setGeneratorAndRegenerateQuery(songsByTagGenerator)
            MusicPlayer.startAndNextIfStarted()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/play/artist/{identifier}") {
            val identifier = call.parameters["identifier"]!!
            val artists = ConfigManager.getAllArtists()
            val artist = artists.find { a -> identifier == a.name || identifier == a.alias }!!
            val songsByIdGenerator = SongGenerator(ConfigManager.getSongsByArtistId(artist.id))
            SongProvider.setGeneratorAndRegenerateQuery(songsByIdGenerator)
            MusicPlayer.startAndNextIfStarted()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/pause") {
            MusicPlayer.pause()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/resume") {
            MusicPlayer.resume()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/repeat") {
            MusicPlayer.repeat()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/loop") {
            MusicPlayer.loop()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/next") {
            MusicPlayer.next()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/prev") {
            MusicPlayer.prev()
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/fastForward/{seconds}") {
            val seconds = call.parameters["seconds"]!!.toLong()
            MusicPlayer.fastForwardXSeconds(seconds)
            call.respond(HttpStatusCode.OK, "OK")
        }

        get("/rewind/{seconds}") {
            val seconds = call.parameters["seconds"]!!.toLong()
            MusicPlayer.rewindXSeconds(seconds)
            call.respond(HttpStatusCode.OK, "OK")
        }
    }
}
