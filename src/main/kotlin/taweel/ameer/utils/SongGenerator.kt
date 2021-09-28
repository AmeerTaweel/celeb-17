package taweel.ameer.utils

import taweel.ameer.models.Song
import java.util.*

class SongGenerator(private val songs: List<Song>) {
    /**
     * Returns songs in [songs] in random order, and with one randomly chosen version of each multi-version song.
     */
    fun generate(): List<Song> {
        return songs.shuffled().chooseOneVersionForMultiVersionSongs()
    }

    /**
     * Returns a new list that is a shuffled version of [this]
     */
    private fun <T> List<T>.shuffled(): List<T> {
        val mutableList = this.toMutableList()
        mutableList.shuffle()
        return Collections.unmodifiableList(mutableList)
    }

    /**
     * Returns a new list that is the same as [this] but with only one version for each multi-version song.
     */
    private fun List<Song>.chooseOneVersionForMultiVersionSongs(): List<Song> {
        val uniqueSongs = mutableListOf<Song>()
        val uniqueSongsIds = mutableListOf<Int>()
        for (song in this) {
            val originalSongId = if (song.versionOf == -1) song.id else song.versionOf
            if (originalSongId !in uniqueSongsIds) {
                uniqueSongs.add(song)
                uniqueSongsIds.add(originalSongId)
            }
        }
        return Collections.unmodifiableList(uniqueSongs)
    }
}