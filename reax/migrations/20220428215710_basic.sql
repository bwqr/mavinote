create table folders(
    id      integer primary key autoincrement,
    name    varchar(255) not null
);

insert into folders (name) values ("Edebiyat"), ("Kimya"), ("Sanat"), ("Matematik");

create table notes(
    id          integer primary key autoincrement,
    folder_id   integer         not null,
    title       varchar(255)    not null,
    text        text            not null,
    foreign key(folder_id)  references  folders(id)
);

insert into notes (folder_id, title, text) values
    (1,"Merhabalar Baslik", "Lorem ipsum, or lipsum as it is sometimes known, is dummy text used in laying out print, graphic or web designs. The passage is attributed to an unknown typesetter in the 15th century who is thought to have scrambled parts of Cicero's De Finibus Bonorum et Malorum for use in a type specimen book. It usually begins with"),
    (2,"Meltem Havasi", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
    (3,"Yuksek Hiz", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Diam donec adipiscing tristique risus nec feugiat. Sed vulputate odio ut enim blandit. Amet dictum sit amet justo donec enim. Sed viverra ipsum nunc aliquet bibendum enim facilisis gravida. Tellus id interdum velit laoreet id donec ultrices tincidunt. Leo urna molestie at elementum eu facilisis sed. Tincidunt tortor aliquam nulla facilisi cras fermentum. Id neque aliquam vestibulum morbi blandit cursus risus at ultrices. Sit amet consectetur adipiscing elit ut. Ultrices eros in cursus turpis massa tincidunt dui ut."),
    (4,"Reax Yapilacaklar", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sagittis eu volutpat odio facilisis mauris sit. Pellentesque eu tincidunt tortor aliquam nulla facilisi cras. Amet justo donec enim diam. Volutpat blandit aliquam etiam erat velit scelerisque. In iaculis nunc sed augue lacus viverra. Nec tincidunt praesent semper feugiat nibh sed pulvinar proin gravida. Pharetra magna ac placerat vestibulum lectus mauris. Sagittis eu volutpat odio facilisis mauris sit. Aliquam purus sit amet luctus venenatis lectus magna fringilla urna.");
