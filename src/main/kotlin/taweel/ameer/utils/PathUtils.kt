package taweel.ameer.utils

object PathUtils {
    fun expandPath(path: String): String {
        return expandHomeDirectoryInPath(path)
    }

    private fun expandHomeDirectoryInPath(path: String): String {
        val homeDirectory = System.getProperty("user.home")
        return path.replaceFirst("^~".toRegex(), homeDirectory)
    }
}