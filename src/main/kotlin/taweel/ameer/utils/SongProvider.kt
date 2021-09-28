package taweel.ameer.utils

import taweel.ameer.models.Song

object SongProvider {
    private val queue: MutableList<Song> = mutableListOf()
    private lateinit var generator: SongGenerator

    fun setGeneratorAndRegenerateQuery(generator: SongGenerator) {
        SongProvider.generator = generator
        queue.clear()
        queue.addAll(generator.generate())
    }

    fun next(): Song {
        if (queue.isEmpty()) queue.addAll(generator.generate())
        return queue.removeFirst()
    }
}