package taweel.ameer.managers

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import taweel.ameer.models.Artist
import taweel.ameer.models.Config
import taweel.ameer.models.Library
import taweel.ameer.models.Song
import taweel.ameer.utils.PathUtils
import java.io.File

object ConfigManager {
    private val config: Config = readConfig()
    private val library: Library = readLibrary()

    object Constants {
        const val configFilePath = "~/.config/g-music-player/gmprc.json"
        const val libraryFilePath = "~/.config/g-music-player/library.json"
    }

    @OptIn(ExperimentalSerializationApi::class)
    private fun readConfig(): Config {
        insureConfigFileExists()
        val configFile = File(PathUtils.expandPath(Constants.configFilePath))
        val configJson = configFile.readText()
        return Json.decodeFromString(configJson)
    }

    private fun insureConfigFileExists() {
        val configFile = File(PathUtils.expandPath(Constants.configFilePath))
        if (!configFile.exists()) createDefaultConfigFile()
    }

    @OptIn(ExperimentalSerializationApi::class)
    private fun createDefaultConfigFile() {
        val defaultConfig = Config()
        val defaultConfigJson = Json.encodeToString(defaultConfig)
        val defaultConfigFile = File(PathUtils.expandPath(Constants.configFilePath))
        defaultConfigFile.parentFile.mkdirs()
        defaultConfigFile.writeBytes(defaultConfigJson.toByteArray())
    }

    @OptIn(ExperimentalSerializationApi::class)
    private fun readLibrary(): Library {
        insureLibraryFileExists()
        val libraryFile = File(PathUtils.expandPath(Constants.libraryFilePath))
        val libraryJson = libraryFile.readText()
        return Json.decodeFromString(libraryJson)
    }

    private fun insureLibraryFileExists() {
        val libraryFile = File(PathUtils.expandPath(Constants.libraryFilePath))
        if (!libraryFile.exists()) createDefaultLibraryFile()
    }

    @OptIn(ExperimentalSerializationApi::class)
    private fun createDefaultLibraryFile() {
        val defaultLibrary = Library()
        val defaultLibraryJson = Json.encodeToString(defaultLibrary)
        val defaultLibraryFile = File(PathUtils.expandPath(Constants.libraryFilePath))
        defaultLibraryFile.parentFile.mkdirs()
        defaultLibraryFile.writeBytes(defaultLibraryJson.toByteArray())
    }

    fun getMusicDirectoryFormat(): String {
        return "${config.musicDirectory.trimEnd('/')}/%s"
    }

    fun getAllSongs(): List<Song> {
        return library.songs
    }

    fun getSongsByTag(tag: String): List<Song> {
        return library.songs.filter { s -> tag in s.tags }
    }

    fun getAllArtists(): List<Artist> {
        return library.artists
    }

    fun getSongsByArtistId(id: Int): List<Song> {
        return library.songs.filter { s -> id == s.artistId }
    }
}