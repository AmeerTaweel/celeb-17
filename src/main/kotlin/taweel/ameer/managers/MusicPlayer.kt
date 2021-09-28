package taweel.ameer.managers

import taweel.ameer.models.Song
import taweel.ameer.utils.SongProvider
import java.io.File
import javax.sound.sampled.AudioSystem
import javax.sound.sampled.Clip
import javax.sound.sampled.LineEvent
import javax.sound.sampled.LineListener
import kotlin.system.exitProcess

object MusicPlayer {
    private enum class State {
        NOT_STARTED, PLAYING, STOPPED, LOOPING
    }

    private val songEndListener: LineListener = LineListener { e ->
        if (e.type != LineEvent.Type.STOP) return@LineListener
        next()
    }
    private val songs: MutableList<Song> = mutableListOf()
    private var state: State = State.NOT_STARTED
    private var index: Int = 0
    private lateinit var clip: Clip

    fun startAndNextIfStarted() {
        if(state == State.NOT_STARTED) {
            state = State.PLAYING
            playSongAtIndex()
        } else next()
    }

    fun pause() {
        if (state == State.STOPPED || state == State.NOT_STARTED) return // Do not pause if no song is running
        state = State.STOPPED
        clip.removeLineListener(songEndListener)
        clip.stop()
    }

    fun resume() {
        if (state != State.STOPPED) return // Do not resume if the song is not paused
        state = State.PLAYING
        clip.start()
        clip.addLineListener(songEndListener)
    }

    fun repeat() {
        if (state == State.STOPPED || state == State.NOT_STARTED) return // Do not repeat if no song is running
        stopPlayingSong()
        playSongAtIndex()
        if (state == State.LOOPING) loop() // Keep the song looping if it was looping
    }

    fun loop() {
        if (state == State.STOPPED || state == State.NOT_STARTED) return // Do not loop if no song is running
        state = State.LOOPING
        clip.loop(Clip.LOOP_CONTINUOUSLY)
    }

    fun fastForwardXSeconds(x: Long){
        val totalTime = clip.microsecondLength
        val currentTime = clip.microsecondPosition
        val newTime = currentTime + x * 1000000L // convert X from second to microsecond
        if(newTime < totalTime) clip.microsecondPosition = newTime
        else next()
    }

    fun rewindXSeconds(x: Long){
        val currentTime = clip.microsecondPosition
        val newTime = currentTime - x * 1000000L // convert X from second to microsecond
        if(newTime > 0) clip.microsecondPosition = newTime
        else repeat()
    }

    fun next() {
        if (state == State.NOT_STARTED) return // Do not go to the next song if the player is not started
        state = State.PLAYING
        stopPlayingSong()
        index++
        playSongAtIndex()
    }

    fun prev() {
        if (state == State.NOT_STARTED) return // Do not go to the previous song if the player is not started
        if (index > 0) {
            state = State.PLAYING
            stopPlayingSong()
            index--
            playSongAtIndex()
        } else repeat() // Repeat the current song if there are no previous ones
    }

    private fun playSongAtIndex() {
        if (index == songs.size) songs.add(SongProvider.next())
        clip = loadSongClip(songs[index])
        clip.start()
        clip.addLineListener(songEndListener)
    }

    private fun stopPlayingSong() {
        clip.removeLineListener(songEndListener)
        clip.close()
    }

    private fun loadSongClip(song: Song): Clip {
        try {
            val clipPath = String.format(ConfigManager.getMusicDirectoryFormat(), "${song.id}.wav")
            val clipAbsolutePath = clipPath.replaceFirst("^~".toRegex(), System.getProperty("user.home"))
            val clipFile = File(clipAbsolutePath)
            val audioIO = AudioSystem.getAudioInputStream(clipFile)
            val clip = AudioSystem.getClip()
            clip.open(audioIO)
            return clip
        } catch (e: Exception) {
            e.printStackTrace()
            exitProcess(1)
        }
    }
}