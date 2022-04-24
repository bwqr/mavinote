package com.bwqr.mavinote.viewmodels

import android.util.Log
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.NoteSummary

val folders = listOf(
    Folder(1, "Gmail Hesabi"),
    Folder(2, "iCloud Hesabi"),
    Folder(3, "Local"),
)

val summaries = listOf(
    NoteSummary(1, 1, "Merhabalar Baslik", "Guzel bir icerik"),
    NoteSummary(2, 1, "Meltem Havasi", "Aksam serinligi"),
    NoteSummary(3, 1, "Yuksek Hiz", "Yuksek devir"),
    NoteSummary(4, 2, "Reax Yapilacaklar", "Yol Haritasi"),
    NoteSummary(5, 2, "MaviNote Yapilacaklar", "Yol Haritasi"),
    NoteSummary(6, 2, "GameDev Yapilacaklar", "GameDev Yol Haritasi"),
    NoteSummary(7, 3, "Edebi agirlikli eserler", "Icerikleri incelemek"),
    NoteSummary(8, 3, "Sanatsal anlamda eserler", "Listelenmesi"),
    NoteSummary(9, 3, "Bilgi toplama", "Arastirilmasi"),
)

var notes = mutableListOf(
    Note(1, 1, "Merhabalar Baslik", "Lorem ipsum, or lipsum as it is sometimes known, is dummy text used in laying out print, graphic or web designs. The passage is attributed to an unknown typesetter in the 15th century who is thought to have scrambled parts of Cicero's De Finibus Bonorum et Malorum for use in a type specimen book. It usually begins with"),
    Note(2, 1, "Meltem Havasi", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
    Note(3, 1, "Yuksek Hiz", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Diam donec adipiscing tristique risus nec feugiat. Sed vulputate odio ut enim blandit. Amet dictum sit amet justo donec enim. Sed viverra ipsum nunc aliquet bibendum enim facilisis gravida. Tellus id interdum velit laoreet id donec ultrices tincidunt. Leo urna molestie at elementum eu facilisis sed. Tincidunt tortor aliquam nulla facilisi cras fermentum. Id neque aliquam vestibulum morbi blandit cursus risus at ultrices. Sit amet consectetur adipiscing elit ut. Ultrices eros in cursus turpis massa tincidunt dui ut."),
    Note(4, 2, "Reax Yapilacaklar", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sagittis eu volutpat odio facilisis mauris sit. Pellentesque eu tincidunt tortor aliquam nulla facilisi cras. Amet justo donec enim diam. Volutpat blandit aliquam etiam erat velit scelerisque. In iaculis nunc sed augue lacus viverra. Nec tincidunt praesent semper feugiat nibh sed pulvinar proin gravida. Pharetra magna ac placerat vestibulum lectus mauris. Sagittis eu volutpat odio facilisis mauris sit. Aliquam purus sit amet luctus venenatis lectus magna fringilla urna."),
    Note(5, 2, "MaviNote Yapilacaklar", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. In cursus turpis massa tincidunt dui. Ut enim blandit volutpat maecenas volutpat blandit aliquam etiam. Tellus at urna condimentum mattis pellentesque id. Velit laoreet id donec ultrices tincidunt. Dolor purus non enim praesent. Amet risus nullam eget felis."),
    Note(6, 2, "GameDev Yapilacaklar", "Lacus sed viverra tellus in. Augue eget arcu dictum varius. In mollis nunc sed id semper risus in hendrerit. Lacus suspendisse faucibus interdum posuere lorem ipsum dolor. Ullamcorper eget nulla facilisi etiam dignissim diam quis enim lobortis. Sed cras ornare arcu dui vivamus arcu felis. Nulla pellentesque dignissim enim sit amet venenatis urna."),
    Note(7, 3, "Edebi agirlikli eserler", "Dui nunc mattis enim ut tellus elementum sagittis. Vel risus commodo viverra maecenas accumsan lacus. Ac tortor dignissim convallis aenean et. Congue mauris rhoncus aenean vel elit scelerisque mauris pellentesque pulvinar. Adipiscing elit duis tristique sollicitudin nibh sit amet commodo nulla."),
    Note(8, 3, "Sanatsal anlamda eserler", "Diam sollicitudin tempor id eu nisl nunc. Duis ultricies lacus sed turpis tincidunt id. Pharetra massa massa ultricies mi. Eget arcu dictum varius duis at consectetur lorem. Nec ultrices dui sapien eget mi proin sed libero. Dignissim suspendisse in est ante in nibh mauris."),
    Note(9, 3, "Bilgi toplama", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Pulvinar elementum integer enim neque volutpat ac tincidunt vitae semper. At consectetur lorem donec massa sapien faucibus et molestie."),
)

class NoteViewModel {
    fun folders(): List<Folder> {
        return folders
    }

    fun notes(folderId: Int): List<NoteSummary> {
        return summaries.filter { it.folderId == folderId }
    }

    fun note(noteId: Int): Note? {
        return notes.find { it.id == noteId }
    }

    fun updateNote(noteId: Int, text: String) {
        Log.d("NoteViewModel", "Note is being updated $noteId")

        val index = notes.withIndex().find { it.value.id == noteId }

        index?.let {
            notes[it.index] = notes[it.index].cloneWithText(text)
        }
    }
}