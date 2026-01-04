--
-- PostgreSQL database dump
--

\restrict opz04Qd9PUWv7QcICoLZxQ1btETzgyZebWWuBF1hJpqaGT8EC0hapXvCHsgkYIo

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: admins; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.admins (
    id integer NOT NULL,
    username text NOT NULL,
    password text NOT NULL
);


ALTER TABLE public.admins OWNER TO postgres;

--
-- Name: admins_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.admins_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.admins_id_seq OWNER TO postgres;

--
-- Name: admins_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.admins_id_seq OWNED BY public.admins.id;


--
-- Name: kuliner; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.kuliner (
    id integer NOT NULL,
    nama_makanan text NOT NULL,
    kategori text NOT NULL,
    alamat text NOT NULL,
    harga integer NOT NULL,
    link_gmaps text NOT NULL,
    link_foto text NOT NULL,
    htm integer,
    deskripsi text,
    fasilitas text[],
    menu_populer text[],
    cocok_untuk text[],
    jam_buka text,
    jam_tutup text,
    trans_kode text,
    trans_jarak_meter integer,
    trans_tarif_min integer,
    trans_tarif_max integer,
    trans_rute text[]
);


ALTER TABLE public.kuliner OWNER TO postgres;

--
-- Name: kuliner_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.kuliner_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.kuliner_id_seq OWNER TO postgres;

--
-- Name: kuliner_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.kuliner_id_seq OWNED BY public.kuliner.id;


--
-- Name: tempat_nongkrong; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tempat_nongkrong (
    id integer NOT NULL,
    nama_tempat text NOT NULL,
    kategori text NOT NULL,
    alamat text NOT NULL,
    jam_buka text NOT NULL,
    jam_tutup text NOT NULL,
    harga_rata_rata integer NOT NULL,
    link_gmaps text NOT NULL,
    link_foto text NOT NULL,
    htm integer,
    deskripsi text,
    fasilitas text[],
    menu_populer text[],
    cocok_untuk text[],
    trans_kode text,
    trans_jarak_meter integer,
    trans_tarif_min integer,
    trans_tarif_max integer,
    trans_rute text[]
);


ALTER TABLE public.tempat_nongkrong OWNER TO postgres;

--
-- Name: tempat_nongkrong_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.tempat_nongkrong_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.tempat_nongkrong_id_seq OWNER TO postgres;

--
-- Name: tempat_nongkrong_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.tempat_nongkrong_id_seq OWNED BY public.tempat_nongkrong.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id integer NOT NULL,
    username text NOT NULL,
    password text NOT NULL,
    email text NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: wisata_alam; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.wisata_alam (
    id integer NOT NULL,
    nama_tempat text NOT NULL,
    kategori text NOT NULL,
    alamat text NOT NULL,
    jam_buka text NOT NULL,
    jam_tutup text NOT NULL,
    htm integer NOT NULL,
    link_gmaps text NOT NULL,
    link_foto text NOT NULL,
    deskripsi text,
    fasilitas text[],
    cocok_untuk text[],
    trans_kode text,
    trans_jarak_meter integer,
    trans_tarif_min integer,
    trans_tarif_max integer,
    trans_rute text[]
);


ALTER TABLE public.wisata_alam OWNER TO postgres;

--
-- Name: wisata_alam_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.wisata_alam_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.wisata_alam_id_seq OWNER TO postgres;

--
-- Name: wisata_alam_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.wisata_alam_id_seq OWNED BY public.wisata_alam.id;


--
-- Name: wisata_pendidikan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.wisata_pendidikan (
    id integer NOT NULL,
    nama_tempat text NOT NULL,
    kategori text NOT NULL,
    alamat text NOT NULL,
    jam_buka text NOT NULL,
    jam_tutup text NOT NULL,
    htm integer NOT NULL,
    link_gmaps text NOT NULL,
    link_foto text NOT NULL,
    deskripsi text,
    fasilitas text[],
    cocok_untuk text[],
    trans_kode text,
    trans_jarak_meter integer,
    trans_tarif_min integer,
    trans_tarif_max integer,
    trans_rute text[]
);


ALTER TABLE public.wisata_pendidikan OWNER TO postgres;

--
-- Name: wisata_pendidikan_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.wisata_pendidikan_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.wisata_pendidikan_id_seq OWNER TO postgres;

--
-- Name: wisata_pendidikan_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.wisata_pendidikan_id_seq OWNED BY public.wisata_pendidikan.id;


--
-- Name: admins id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.admins ALTER COLUMN id SET DEFAULT nextval('public.admins_id_seq'::regclass);


--
-- Name: kuliner id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.kuliner ALTER COLUMN id SET DEFAULT nextval('public.kuliner_id_seq'::regclass);


--
-- Name: tempat_nongkrong id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tempat_nongkrong ALTER COLUMN id SET DEFAULT nextval('public.tempat_nongkrong_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: wisata_alam id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wisata_alam ALTER COLUMN id SET DEFAULT nextval('public.wisata_alam_id_seq'::regclass);


--
-- Name: wisata_pendidikan id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wisata_pendidikan ALTER COLUMN id SET DEFAULT nextval('public.wisata_pendidikan_id_seq'::regclass);


--
-- Data for Name: admins; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.admins (id, username, password) FROM stdin;
1	admin	$2b$12$7CK.emQvZwtNZvwh8FvZD.h5FbJeNU1v8nKpIA9ET/9IdUAbZylya
\.


--
-- Data for Name: kuliner; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.kuliner (id, nama_makanan, kategori, alamat, harga, link_gmaps, link_foto, htm, deskripsi, fasilitas, menu_populer, cocok_untuk, jam_buka, jam_tutup, trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute) FROM stdin;
1	Mendoan Banyumas	Kuliner Tradisional	Jl. Jend. Sudirman, Purwokerto	5000	https://www.google.com/maps/search/?api=1&query=Mendoan+Purwokerto	/uploads/seed/mendoan.jpg	\N	Mendoan khas Banyumas, cocok buat ngemil.	{parking,indoor}	{Mendoan,"Es Teh"}	{"Kuliner lokal",Cemilan}	07.00	20.00	Koridor Kota	400	3000	5000	{Alun-Alun,"Pasar Wage"}
2	Soto Sokaraja	Kuliner Tradisional	Sokaraja, Banyumas	15000	https://www.google.com/maps/search/?api=1&query=Soto+Sokaraja	/uploads/seed/soto-sokaraja.jpg	\N	Soto khas Sokaraja dengan sambal kacang.	{parking,toilet}	{Soto,Kerupuk,Teh}	{Sarapan,Keluarga}	06.00	15.00	Koridor Sokaraja	600	4000	6000	{"Terminal Sokaraja",Sokaraja}
3	Getuk Goreng Sokaraja	Oleh-oleh	Jl. Raya Sokaraja, Banyumas	12000	https://www.google.com/maps/search/?api=1&query=Getuk+Goreng+Sokaraja	/uploads/seed/getuk-goreng.jpg	\N	Oleh-oleh khas Banyumas berbahan singkong.	{parking}	{"Getuk Goreng"}	{Oleh-oleh,Keluarga}	08.00	20.00	Koridor Sokaraja	500	4000	6000	{Sokaraja,Purwokerto}
4	Nasi Nyangku	Kuliner Lokal	Purwokerto	18000	https://www.google.com/maps/search/?api=1&query=Nasi+Nyangku+Purwokerto	/uploads/seed/nasi-nyangku.jpg	\N	Menu nasi dengan lauk rumahan khas.	{parking,indoor}	{"Nasi Nyangku","Es Jeruk"}	{"Makan siang",Keluarga}	10.00	21.00	Koridor Kota	700	3000	5000	{Alun-Alun,Karangwangkal}
5	Bakso Favorite Purwokerto	Bakso	Purwokerto	17000	https://www.google.com/maps/search/?api=1&query=Bakso+Purwokerto	/uploads/seed/bakso.jpg	\N	Bakso kuah gurih dengan pilihan mie/bihun.	{parking,toilet}	{"Bakso Urat","Bakso Telur","Es Teh"}	{"Makan siang",Nongkrong}	10.00	22.00	Koridor Kota	650	3000	5000	{"Pasar Wage",Alun-Alun}
7	Mie Ayam Pangsit	Mie Ayam	Purwokerto	14000	https://www.google.com/maps/search/?api=1&query=Mie+Ayam+Purwokerto	/uploads/seed/mie-ayam.jpg	\N	Mie ayam dengan pangsit goreng renyah.	{parking,indoor}	{"Mie Ayam Pangsit","Es Teh"}	{"Makan siang",Cepat}	09.00	20.00	Koridor Kota	500	3000	5000	{"HR Bunyamin",Unsoed}
8	Es Dawet Ayu	kuliner	Purwokerto	8000	https://www.google.com/maps/search/?api=1&query=Es+Dawet+Ayu+Purwokerto	http://localhost:7860/uploads/97815723-c25c-4586-ba5c-e0a3991cd448.png	\N	Minuman segar dawet dengan gula merah.	{parking,outdoor}	{"Es Dawet AYu"}	{Seger,Cemilan}	09.00	17.00	Koridor Kota	350	3000	5000	{Alun-Alun,Pasar}
6	Ayam Goreng Kalasan Pak Bambang	kuliner	Purwokerto Wetan, Kec. Purwokerto Tim., Kabupaten Banyumas, Jawa Tengah 53147	37500	https://www.google.com/maps/search/?api=1&query=Ayam+Goreng+Banyumas	http://localhost:7860/uploads/d00dcdae-01ae-404c-b1d7-1eeb2e4182cb.png	\N	Ayam goreng manis gurih ala Jawa.	{parking,indoor,toilet,family,halal}	{"Ayam Goreng",Sambal,Kremes}	{"Makan malam",Keluarga}	08:00	21:00	Koridor Banyumas	900	3500	6000	{Banyumas,Purwokerto}
\.


--
-- Data for Name: tempat_nongkrong; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tempat_nongkrong (id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, harga_rata_rata, link_gmaps, link_foto, htm, deskripsi, fasilitas, menu_populer, cocok_untuk, trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute) FROM stdin;
11	Pagi Hari Coffce	cafe	Jl. Soeparjo Roestam No.4, Purwokerto	07.00	21.00	26500	https://www.google.com/maps/search/?api=1&query=Pagi+Hari+Coffee+Purwokerto	/uploads/seed/pagi-hari-coffce.jpg	\N	Cafe yang buka lebih pagi, cocok untuk sarapan ringan dengan kopi hangat.	{wifi,ac,parking}	{"Kopi Hitam Pagi",Latte,"Roti Bakar"}	{Sarapan,"Kerja Pagi"}	TB-01	300	3500	5000	{Ajibarang,"Pasar Pon","Soeparjo Roestam"}
12	Sisi Timur Cafe	cafe	Jl. KH Ahmad Dahlan No.19, Purwokerto	09.00	22.00	32500	https://www.google.com/maps/search/?api=1&query=Sisi+Timur+Cafe+Purwokerto	/uploads/seed/sisi-timur-cafe.jpg	\N	Cafe dengan jendela besar menghadap timur, cahaya pagi yang masuk bikin suasana hangat.	{wifi,ac,parking,photoSpot}	{"Es Kopi Timur",Americano,"Snack Box"}	{"Kerja / Belajar","Nongkrong Pagi"}	TB-03	250	3500	5000	{Bulupitu,"KH Ahmad Dahlan",Alun-Alun,Kebondalem}
2	Cold 'N Brew	tempat_nongkrong	Jl. Jend. Sudirman No.298, Purwokerto	08:00	22:00	37500	https://www.google.com/maps/search/?api=1&query=Cold+N+Brew+Purwokerto	http://localhost:7860/uploads/6eae11e1-0cfe-4dd3-ab87-6375af935c5c.jpg	\N	Cafe dengan interior hangat, menu kopi dan non-kopi lengkap. Nyaman buat kerja, ngobrol, atau sekadar mampir sebentar.	{wifi,socket,ac,parking,24h}	{"Cold Brew Signature","Caramel Latte","Hazelnut Coffee"}	{"Kerja / Belajar","Nongkrong Larut Malam"}	TB-03	150	3500	5000	{"Terminal Bulupitu","GOR Satria",Alun-Alun,Kebondalem}
3	Advo Cafe	tempat_nongkrong	Jl. A. Yani No.60, Karangwangkal, Kec. Purwokerto Utara, Purwokerto	10:00	23:00	17500	https://www.google.com/maps/search/?api=1&query=Advo+Cafe+Purwokerto	http://localhost:7860/uploads/05437d35-fe72-4c30-a01a-d325cef7faec.jpg	\N	Cafe dengan view hijau dan suasana tenang. Cocok buat healing tipis-tipis sambil nugas atau ngobrol santai.	{wifi,socket,ac,parking,outdoor,photoSpot}	{"Signature Latte","Matcha Latte","Cold Brew","Snack Platter"}	{"Kerja / Belajar",Nongkrong,"Me Time"}	TB-03	250	3500	5000	{"Terminal Bulupitu",Unsoed,Karangwangkal,Kebondalem}
4	My Story Cafe & Bistro Purwokerto	tempat_nongkrong	Jl. S. Parman No.638, Kedungampel, Purwokerto Kulon, Kec. Purwokerto Sel., Kabupaten Banyumas, Jawa Tengah 53141	10:00	23:00	50000	https://www.google.com/maps/search/?api=1&query=Kedai+Sore+Cafe+Purwokerto	http://localhost:7860/uploads/d861921b-6de3-4321-a558-7f99e6650115.jpg	\N	Tempat nongkrong dengan suasana hangat, cocok untuk ngobrol santai bareng teman di waktu sore hingga malam.	{wifi,parking,outdoor,photoSpot}	{"Es Kopi Susu","Teh Manis Panas","Gorengan Platter"}	{"Nongkrong Sore","Buka Puasa","Kumpul Teman"}	TB-01	300	3500	5000	{"Pasar Pon","S. Parman",Alun-Alun,"Terminal Ajibarang"}
5	Senja coffe purwokerto	tempat_nongkrong	Jl. Tegal Mulya, RT.1RT 004/RW.05, Dusun III, Ledug, Kec. Kembaran, Kabupaten Banyumas, Jawa Tengah 53111	09:00	23:00	35000	https://www.google.com/maps/search/?api=1&query=Ruang+Senja+Coffee+Purwokerto	http://localhost:7860/uploads/ee815a68-5740-476c-8953-496bfaef0994.jpg	\N	Cafe minimalis dengan banyak colokan dan wifi kencang. Favorit mahasiswa buat nugas sampai malam.	{wifi,socket,parking}	{"Es Kopi Senja","Choco Latte","Camilan Ringan"}	{"Kerja / Belajar","Nongkrong Malam"}	TB-03	150	3500	5000	{"Terminal Bulupitu","HR Bunyamin","GOR Satria",Kebondalem}
6	Angkringan Kafe Copi Langitan	tempat_nongkrong	H67H+8HQ, Bojong, Tanjung, Kec. Purwokerto Sel., Kabupaten Banyumas, Jawa Tengah 53144	06:30	01:00	12501	https://www.google.com/maps/search/?api=1&query=Langit+Kopi+Purwokerto	http://localhost:7860/uploads/d49dd96d-ff1d-4c5a-8f3d-9cd07a1d6e70.jpg	\N	Rooftop cafe dengan pemandangan kota Purwokerto, cocok buat foto-foto dan nongkrong malam.	{wifi,parking,outdoor,photoSpot}	{"Kopi Susu Langit",Mocktail,"Snack Sharing"}	{"Nongkrong Malam",Foto-foto,Date}	TB-02	1400	3500	5000	{"Terminal RS margono","Jalan Baru"}
8	Teratai Cafe	tempat_nongkrong	Jl. Bung Karno, Kalibener, Purwanegara, Kec. Purwokerto Tim., Kabupaten Banyumas, Jawa Tengah 53114	09:30	22:00	37500	https://maps.app.goo.gl/UDGBjenT4resTfQP9	http://localhost:7860/uploads/75bd671b-61aa-4b91-aeb7-ebe72b1a261f.jpg	\N	cafe dengan pemandangan khas kota purwokerto	{wifi,parking}	{"Es Kopi Jalan","Kopi Susu Gula Aren","French Fries"}	{"Nongkrong Cepat","Take Away"}	\N	\N	\N	\N	{}
9	teras coffee & eatery	tempat_nongkrong	Jl. Prof. Dr. Suharso No.49, Mangunjaya, Purwokerto Lor, Kec. Purwokerto Tim., Kabupaten Banyumas, Jawa Tengah 53114	09:00	00:00	37500	https://www.google.com/maps/search/?api=1&query=Teras+Kopi+Purwokerto	http://localhost:7860/uploads/d5bcb0be-20c0-4379-ae0a-982600240ff1.jpg	\N	Cafe outdoor dengan banyak area duduk di teras, cocok untuk nongkrong sore bareng teman.	{wifi,parking,outdoor,socket}	{"Es Kopi Teras","Teh Tarik","Snack Goreng"}	{"Nongkrong Sore","Ngobrol Santai"}	TB-01	250	3500	5000	{"Pasar Pon","Dr. Angka",Alun-Alun,Ajibarang}
10	Kopi Sudut Kota	tempat_nongkrong	Jl. Jend. Sudirman, Purwokerto, Purwokerto Lor, Kec. Purwokerto Tim., Kabupaten Banyumas, Jawa Tengah 53148	19:00	01:00	12501	https://maps.app.goo.gl/bUTELNP1j6jysgPf7	http://localhost:7860/uploads/1f282b31-3a66-4fe2-9b0d-2f9df8cab5b0.jpg	\N	cafe pinggir jalan dengan view jalan yang aesthetic	{parking,photoSpot}	{"Kopi Susu Kota",Mocha,"Dessert Slice"}	{Nongkrong,Foto-foto}	TB-03	300	3500	5000	{Bulupitu,"Overste Isdiman",Alun-Alun,Kebondalem}
13	Kopi Kolcktif	cafe	Jl. Tentara Pelajar No.2, Purwokerto	09.00	22.00	42500	https://www.google.com/maps/search/?api=1&query=Kopi+Kolektif+Purwokerto	/uploads/seed/kopi-kolcktif.jpg	\N	Co-working cafe yang menyediakan ruang kerja bersama dengan fasilitas lengkap.	{wifi,socket,ac,parking,coworking}	{"Kopi Kolektif","Manual Brew","Snack Sharing"}	{"Kerja Tim",Freelancer,"Meeting Kecil"}	TB-03	200	3500	5000	{Bulupitu,"Tentara Pelajar",Alun-Alun,Kebondalem}
14	Ruang Teduh	cafe	Jl. Merdeka No.17, Purwokerto	10.00	22.00	29000	https://www.google.com/maps/search/?api=1&query=Ruang+Teduh+Purwokerto	/uploads/seed/ruang-teduh.jpg	\N	Cafe dengan interior earthy dan musik lembut, cocok untuk yang cari ketenangan.	{wifi,ac,parking,quietPlace,photoSpot}	{"Latte Teduh","Chamomile Tea",Cookies}	{"Me Time","Baca Buku"}	TB-01	250	3500	5000	{Ajibarang,"Pasar Pon",Merdeka}
15	Kopi Kampus	cafe	Jl. Kampus Raya No.1, Purwokerto	09.00	23.00	22500	https://www.google.com/maps/search/?api=1&query=Kopi+Kampus+Purwokerto	/uploads/seed/kopi-kampus.jpg	\N	Cafe dekat kampus dengan harga ramah mahasiswa dan banyak colokan di setiap sudut.	{wifi,socket,ac,parking}	{"Kopi Susu Kampus","Es Teh Manis","Mie Rebus"}	{"Kerja / Belajar","Nongkrong Mahasiswa"}	TB-02	150	3500	5000	{Notog,Kampus,Unsoed,Baturraden}
16	Sore di Kota	cafe	Jl. Jend. Gatot Subroto No.8, Purwokerto	16.00	23.00	33500	https://www.google.com/maps/search/?api=1&query=Sore+di+Kota+Purwokerto	/uploads/seed/sore-di-kota.jpg	\N	Cafe dengan view jalan utama, enak buat menikmati suasana kota menjelang malam.	{wifi,parking,outdoor,photoSpot}	{"Es Kopi Sore",Mocktail,"Snack Platter"}	{"Nongkrong Sore",Foto-foto}	TB-03	250	3500	5000	{Bulupitu,"Gatot Subroto",Alun-Alun,Kebondalem}
17	Garden Brew	cafe	Jl. Beji No.29, Purwokerto	10.00	22.00	35000	https://www.google.com/maps/search/?api=1&query=Garden+Brew+Purwokerto	/uploads/seed/garden-brew.jpg	\N	Cafe bernuansa taman dengan banyak tanaman dan area duduk outdoor.	{wifi,parking,outdoor,photoSpot}	{"Kopi Garden","Lemon Tea","Snack Goreng"}	{Nongkrong,"Family Time"}	TB-02	300	3500	5000	{Notog,Beji,"RS Margono",Baturraden}
18	Kopi Tengah Kota	cafe	Jl. Raya Tengah No.3, Purwokerto	09.00	23.00	32500	https://www.google.com/maps/search/?api=1&query=Kopi+Tengah+Kota+Purwokerto	/uploads/seed/kopi-tengah-kota.jpg	\N	Lokasi strategis di pusat kota, mudah dijangkau dan dekat dengan banyak tempat penting.	{wifi,ac,parking}	{"Kopi Susu Tengah",Americano,"Snack Ringan"}	{"Meeting Santai",Nongkrong}	TB-03	150	3500	5000	{Bulupitu,"Tengah Kota",Alun-Alun,Kebondalem}
19	Senja & Rasa	cafe	Jl. Kalibener No.11, Purwokerto	15.00	23.00	29000	https://www.google.com/maps/search/?api=1&query=Senja+%26+Rasa+Purwokerto	/uploads/seed/senja-rasa.jpg	\N	Cafe yang terkenal dengan menu kopi susu gula aren dan suasana senja yang hangat.	{wifi,parking,outdoor,photoSpot}	{"Kopi Senja","Kopi Susu Gula Aren","Snack Manis"}	{"Nongkrong Sore",Date}	TB-01	250	3500	5000	{Ajibarang,Kalibener,"Pasar Pon"}
20	Kopi Tepi Sawah	cafe	Jl. Raya Patikraja No.5, Banyumas	09.00	21.00	35000	https://www.google.com/maps/search/?api=1&query=Kopi+Tepi+Sawah+Banyumas	/uploads/seed/kopi-tepi-sawah.jpg	\N	Cafe dengan pemandangan persawahan, cocok untuk melepas penat dari suasana kota.	{wifi,parking,outdoor,natureView}	{"Kopi Sawah","Teh Hangat","Pisang Goreng"}	{"Family Time","Me Time"}	TB-02	400	3500	5000	{"Terminal Notog",Patikraja,Purwokerto,Baturraden}
1	Kopi Calf	tempat_nongkrong	Jl. Prof. Dr. Suharso No.53, Karangwangkal, Purwokerto	08:00	23:00	37500	https://www.google.com/maps/search/?api=1&query=Kopi+Calf+Purwokerto	http://localhost:7860/uploads/15a66b6c-7308-46fe-9fd0-d7bec1e25ba3.jpg	\N	Cafe cozy dengan suasana nyaman dan kopi specialty yang mantap. Enak buat nugas, meeting kecil, atau sekadar santai bareng teman.	{wifi,socket,ac,parking,outdoor,musholla}	{"Calf Premium","Smooth Series","Magic Tiramisu Cream",Brewmalt}	{"Kerja / Belajar",Nongkrong,Date}	TB-03	200	3500	5000	{"Terminal Bulupitu","HR Bunyamin",Alun-Alun,"Pasar Wage"}
22	Arasta Alpha, Overste Isdiman	tempat_nongkrong	Ruko Widodo, Jl. Overste Isdiman, Jatiwinangun, Purwokerto Lor, Kec. Purwokerto Tim., Kabupaten Banyumas, Jawa Tengah 53114	23:59	22:00	25000	Ruko Widodo, Jl. Overste Isdiman, Jatiwinangun, Purwokerto Lor, Kec. Purwokerto Tim., Kabupaten Banyumas, Jawa Tengah 53114	http://localhost:7860/uploads/f8bb14b9-9c2d-4db0-8d77-6e4188c5d771.jpg	\N	tempat nongkrong aesthetic anak hedon purwokerto	{wifi,ac,parking,socket,24h,studyFriendly}	{"kopi arasta"}	{mahasiswa}	\N	\N	\N	\N	{}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, username, password, email) FROM stdin;
\.


--
-- Data for Name: wisata_alam; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.wisata_alam (id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi, fasilitas, cocok_untuk, trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute) FROM stdin;
1	Curug Cipendok	Wisata Alam	Dusun III Lebaksiu, Karanganyar, Kec. Cilongok, Kabupaten Banyumas	08.30	17.00	15000	https://www.google.com/maps/search/?api=1&query=Curug+Cipendok	/uploads/seed/curug-cipendok.jpg	Air terjun tertinggi di Banyumas dengan ketinggian sekitar 92 meter dan suasana hutan pinus yang sejuk.	{"Area parkir","Warung makan",Toilet,Gazebo}	{"Liburan keluarga",Healing,Fotografi}	Koridor 1	30000	4000	8000	{"Terminal Bulupitu",Cilongok,"Curug Cipendok"}
2	Wisata Alam Jenggala	Wisata Alam	Jl. Pangeran Limboro, Ketenger, Baturraden	07.30	18.30	22500	https://www.google.com/maps/search/?api=1&query=Wisata+Alam+Jenggala	/uploads/seed/wisata-alam-jenggala.jpg	Destinasi alam dengan air terjun mini, sungai jernih, dan spot foto outdoor.	{"Area parkir","Food stall",Toilet,"Spot foto",Mushola}	{"Wisata keluarga","Couple trip","Content creator"}	Koridor 3	25000	4000	8000	{"Terminal Bulupitu",Baturraden,"Wisata Alam Jenggala"}
3	Owabong Water Park	Wisata Alam	Jl. Raya Owabong No.1, Bojongsari, Purbalingga	08.00	18.00	23000	https://www.google.com/maps/search/?api=1&query=Owabong+Water+Park	/uploads/seed/owabong.jpg	Taman bermain air populer dengan berbagai wahana seluncuran dan kolam.	{"Area parkir luas","Food court",Toilet,Loker,Mushola}	{"Family trip",Anak-anak,"Rombongan sekolah"}	Koridor Purbalingga	60000	5000	15000	{Purwokerto,Purbalingga,Owabong}
4	Curug Cipendok	Wisata Alam	Cilongok, Banyumas	08.00	17.00	15000	https://www.google.com/maps/search/?api=1&query=Curug+Cipendok	/uploads/seed/curug-cipendok.jpg	Air terjun tinggi dengan suasana hutan pinus.	{parkir,toilet,warung}	{Healing,Fotografi,Keluarga}	Koridor Cilongok	30000	4000	8000	{Bulupitu,Cilongok,Cipendok}
5	Wisata Alam Jenggala	Wisata Alam	Ketenger, Baturraden	07.00	18.30	22500	https://www.google.com/maps/search/?api=1&query=Wisata+Alam+Jenggala	/uploads/seed/wisata-alam-jenggala.jpg	Sungai jernih dan spot foto outdoor.	{parkir,toilet,musholla,spot-foto}	{Keluarga,Couple,Content}	Koridor Baturraden	25000	4000	8000	{Bulupitu,Baturraden,Jenggala}
6	Owabong Water Park	Wisata Alam	Bojongsari, Purbalingga	08.00	18.00	23000	https://www.google.com/maps/search/?api=1&query=Owabong+Water+Park	/uploads/seed/owabong.jpg	Waterpark populer untuk keluarga.	{parkir,food-court,toilet,loker}	{Keluarga,Anak-anak,Rombongan}	Koridor Purbalingga	60000	5000	15000	{Purwokerto,Purbalingga,Owabong}
7	Lokawisata Baturraden	Wisata Alam	Baturraden, Banyumas	07.00	17.00	25000	https://www.google.com/maps/search/?api=1&query=Lokawisata+Baturaden	/uploads/seed/lokawisata-baturraden.jpg	Taman rekreasi alam dengan udara sejuk.	{parkir,toilet,warung}	{Keluarga,Healing}	Koridor Baturraden	35000	4000	8000	{Bulupitu,Baturraden,Lokawisata}
8	Telaga Sunyi	Wisata Alam	Karangmangu, Baturraden	07.00	17.00	15000	https://www.google.com/maps/search/?api=1&query=Telaga+Sunyi	/uploads/seed/telaga-sunyi.jpg	Kolam alami jernih di tengah hutan.	{parkir,toilet}	{Healing,Snorkeling,Fotografi}	Koridor Baturraden	32000	4000	8000	{Bulupitu,Baturraden,Karangmangu}
9	Curug Jenggala	Wisata Alam	Ketenger, Baturraden	07.00	17.00	15000	https://www.google.com/maps/search/?api=1&query=Curug+Jenggala	/uploads/seed/curug-jenggala.jpg	Air terjun dengan jembatan & spot foto.	{parkir,toilet,spot-foto}	{Fotografi,Keluarga}	Koridor Baturraden	33000	4000	8000	{Bulupitu,Baturraden,Ketenger}
10	Curug Bayan	Wisata Alam	Baturraden, Banyumas	07.00	17.00	10000	https://www.google.com/maps/search/?api=1&query=Curug+Bayan	/uploads/seed/curug-bayan.jpg	Curug dekat akses utama Baturraden.	{parkir,toilet}	{Keluarga,Healing}	Koridor Baturraden	28000	4000	8000	{Bulupitu,Baturraden,Bayan}
11	Hutan Pinus Limpakuwus	Wisata Alam	Limpakuwus, Baturraden	07.00	17.00	15000	https://www.google.com/maps/search/?api=1&query=Hutan+Pinus+Limpakuwus	/uploads/seed/limpakuwus.jpg	Hutan pinus untuk piknik & foto.	{parkir,toilet,spot-foto}	{Piknik,Fotografi,Keluarga}	Koridor Baturraden	36000	4000	8000	{Bulupitu,Baturraden,Limpakuwus}
12	Kebun Raya Baturraden	Wisata Alam	Baturraden, Banyumas	08.00	16.00	20000	https://www.google.com/maps/search/?api=1&query=Kebun+Raya+Baturraden	/uploads/seed/kebun-raya.jpg	Kebun raya sejuk, cocok jalan santai.	{parkir,toilet}	{Healing,Keluarga}	Koridor Baturraden	37000	4000	8000	{Bulupitu,Baturraden,KebunRaya}
13	Bukit Tranggulasih	Wisata Alam	Banyumas	06.00	18.00	10000	https://www.google.com/maps/search/?api=1&query=Bukit+Tranggulasih	/uploads/seed/tranggulasih.jpg	View sunrise & citylight.	{parkir,spot-foto}	{Sunrise,Fotografi}	Koridor Banyumas	45000	4000	9000	{Purwokerto,Banyumas,Bukit}
14	Slamet Mountain View Point	Wisata Alam	Sekitar Baturraden	05.00	18.00	15000	https://www.google.com/maps/search/?api=1&query=Viewpoint+Gunung+Slamet+Baturaden	/uploads/seed/slamet-view.jpg	Spot melihat Gunung Slamet & kabut pagi.	{parkir,spot-foto}	{Sunrise,Healing}	Koridor Baturraden	42000	4000	9000	{Bulupitu,Baturraden,Viewpoint}
15	Pancuran 7 Baturraden	Wisata Alam	Baturraden, Banyumas	07.00	17.00	20000	https://www.google.com/maps/search/?api=1&query=Pancuran+7+Baturaden	/uploads/seed/pancuran-7.jpg	Pemandian air panas alami.	{parkir,toilet,warung}	{Healing,Keluarga}	Koridor Baturraden	34000	4000	8000	{Bulupitu,Baturraden,Pancuran7}
\.


--
-- Data for Name: wisata_pendidikan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.wisata_pendidikan (id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto, deskripsi, fasilitas, cocok_untuk, trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute) FROM stdin;
\.


--
-- Name: admins_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.admins_id_seq', 1, true);


--
-- Name: kuliner_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.kuliner_id_seq', 10, true);


--
-- Name: tempat_nongkrong_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.tempat_nongkrong_id_seq', 22, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.users_id_seq', 1, false);


--
-- Name: wisata_alam_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.wisata_alam_id_seq', 15, true);


--
-- Name: wisata_pendidikan_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.wisata_pendidikan_id_seq', 1, false);


--
-- Name: admins admins_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.admins
    ADD CONSTRAINT admins_pkey PRIMARY KEY (id);


--
-- Name: admins admins_username_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.admins
    ADD CONSTRAINT admins_username_key UNIQUE (username);


--
-- Name: kuliner kuliner_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.kuliner
    ADD CONSTRAINT kuliner_pkey PRIMARY KEY (id);


--
-- Name: tempat_nongkrong tempat_nongkrong_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tempat_nongkrong
    ADD CONSTRAINT tempat_nongkrong_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: wisata_alam wisata_alam_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wisata_alam
    ADD CONSTRAINT wisata_alam_pkey PRIMARY KEY (id);


--
-- Name: wisata_pendidikan wisata_pendidikan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wisata_pendidikan
    ADD CONSTRAINT wisata_pendidikan_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

\unrestrict opz04Qd9PUWv7QcICoLZxQ1btETzgyZebWWuBF1hJpqaGT8EC0hapXvCHsgkYIo

