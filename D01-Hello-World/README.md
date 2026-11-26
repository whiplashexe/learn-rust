# 🦀 Day 01: Hello World & Dasar Rust

> **Target hari ini:** paham cara kerja Cargo, variabel, tipe data dasar, function, dan kontrol alur, lalu menyelesaikan challenge FizzBuzz.

## Daftar Isi
1. [Cargo](#1-cargo)
2. [Struktur Program Rust](#2-struktur-program-rust)
3. [Print & Format Output](#3-print--format-output)
4. [Komentar](#4-komentar)
5. [Variabel](#5-variabel)
6. [Konstanta](#6-konstanta)
7. [Tipe Data](#7-tipe-data)
8. [Function](#8-function)
9. [Statement vs Expression](#9-statement-vs-expression)
10. [Kontrol Alur](#10-kontrol-alur)
11. [Membaca Pesan Error](#11-membaca-pesan-error)
12. [Rangkuman](#12-rangkuman)
13. [Challenge](#13-challenge)

---

## 1. Cargo

### Definisi
**Cargo** adalah *build system* sekaligus *package manager* bawaan Rust. Fungsinya mirip `npm` di JavaScript atau `composer` di PHP.

### Apa saja yang bisa dilakukan
| Perintah | Fungsi |
|---|---|
| `cargo new nama` | Membuat project baru (sekaligus `git init`) |
| `cargo check` | Mengecek error **tanpa** menghasilkan program. Paling cepat, dipakai saat ngoding |
| `cargo build` | Compile ke `target/debug/` (cepat di-compile, programnya belum dioptimasi) |
| `cargo build --release` | Compile ke `target/release/` (lebih lama di-compile, programnya jauh lebih cepat) |
| `cargo run` | Build lalu langsung jalankan |
| `cargo fmt` | Merapikan format kode otomatis |
| `cargo clippy` | Linter yang memberi saran supaya kode lebih idiomatik |
| `cargo add nama_crate` | Menambah library (dependency) |
| `cargo test` | Menjalankan test |

### Struktur project
```
D01-Hello-World/
├── Cargo.toml     # metadata project + daftar dependency
├── Cargo.lock     # versi pasti dependency (dibuat otomatis)
├── .gitignore     # berisi /target
├── src/
│   └── main.rs    # titik masuk program
└── target/        # hasil compile (jangan di-commit)
```

### Isi `Cargo.toml`
```toml
[package]
name = "D01-Hello-World"   # nama project
version = "0.1.0"          # versi (semantic versioning)
edition = "2024"           # edisi bahasa Rust yang dipakai

[dependencies]             # library tambahan ditulis di sini
```

> 📝 **Edition** bukan versi compiler. Edition (2015, 2018, 2021, 2024) adalah "dialek" Rust. Compiler terbaru tetap bisa meng-compile semua edition.

---

## 2. Struktur Program Rust

```rust
fn main() {
    println!("Hello, world!");
}
```

| Bagian | Arti |
|---|---|
| `fn` | Kata kunci untuk membuat function |
| `main` | Function khusus, **program selalu mulai dari sini** |
| `{ }` | Body function |
| `println!` | **Macro** (ditandai `!`) untuk mencetak teks + baris baru |
| `;` | Akhir statement |

> 📝 **Macro vs function:** macro ditandai `!` dan bisa menerima jumlah argumen yang berubah-ubah. Contoh lain: `print!`, `format!`, `vec!`, `panic!`.

---

## 3. Print & Format Output

```rust
let nama = "Apriana";
let umur = 25;

println!("Halo");                      // teks biasa
println!("Nama: {}", nama);            // placeholder berurutan
println!("Nama: {nama}, umur {umur}"); // nama variabel langsung di dalam {}
println!("{0} {1} {0}", "a", "b");     // berdasarkan posisi → a b a
println!("{:?}", (1, "dua"));          // Debug format → (1, "dua")
println!("{:#?}", (1, "dua"));         // Debug format versi rapi (multi-baris)
println!("{:.2}", 3.14159);            // 2 angka di belakang koma → 3.14
println!("[{:>5}]", 42);               // rata kanan, lebar 5 → [   42]
println!("[{:<5}]", 42);               // rata kiri → [42   ]
println!("[{:^5}]", 42);               // rata tengah → [ 42  ]
println!("{:05}", 42);                 // isi dengan nol → 00042
println!("{{ }}");                     // mencetak kurung kurawal → { }

print!("tanpa baris baru");            // tanpa \n
eprintln!("pesan error");              // ke stderr

let teks = format!("Halo {nama}");     // hasilnya String, tidak dicetak
```

---

## 4. Komentar

```rust
// Komentar satu baris

/* Komentar
   banyak baris */

/// Doc comment: dokumentasi untuk item di bawahnya (function, struct, dll).
/// Bisa dibuka sebagai halaman dokumentasi dengan `cargo doc --open`
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 5. Variabel

### Definisi
**Variabel** adalah nama yang terikat (*binding*) ke sebuah nilai. Dibuat dengan kata kunci `let`.

```rust
let nama = "Apriana";
```

### Konsep penting

#### a. Immutable secara default
Variabel di Rust **tidak bisa diubah** setelah dibuat.

```rust
let x = 5;
x = 6; // ❌ error[E0384]: cannot assign twice to immutable variable `x`
```

> 💡 **Kenapa?** Supaya aman. Kalau sebuah nilai tidak seharusnya berubah, compiler yang menjaganya, bukan ingatan programmer.

#### b. `mut`: agar bisa diubah
```rust
let mut skor = 0;
skor = 10;      // ✅
skor += 5;      // ✅ sekarang 15
```

⚠️ `mut` hanya mengizinkan **nilai** berubah, bukan **tipe**:
```rust
let mut x = 5;
x = "lima"; // ❌ error: expected integer, found `&str`
```

#### c. Type inference & type annotation
Rust bisa **menebak tipe** dari nilainya, tapi kamu juga bisa menuliskannya secara eksplisit.

```rust
let a = 10;          // ditebak sebagai i32
let b: u8 = 10;      // ditulis eksplisit sebagai u8
let c = 10u64;       // tipe ditulis sebagai suffix
let d = 3.0;         // ditebak sebagai f64
```

Kadang tipe **wajib** ditulis karena compiler tidak bisa menebak:
```rust
let angka: i32 = "42".parse().unwrap(); // tanpa `: i32`, compiler bingung mau parse ke tipe apa
```

#### d. Shadowing
Membuat variabel **baru** dengan nama yang **sama**. Variabel lama "tertutupi".

```rust
let x = 5;
let x = x + 1;        // x baru = 6
{
    let x = x * 2;    // x di dalam blok = 12
    println!("{x}");  // 12
}
println!("{x}");      // 6 (shadowing di dalam blok sudah berakhir)
```

Shadowing **boleh mengganti tipe**, berbeda dengan `mut`:
```rust
let input = "  42  ";               // &str
let input = input.trim();           // &str
let input: i32 = input.parse().unwrap(); // i32
```

| | `mut` | Shadowing |
|---|---|---|
| Pakai `let` lagi? | Tidak | Ya |
| Bisa ganti tipe? | ❌ | ✅ |
| Variabel baru? | Tidak, variabel sama | Ya, variabel baru |
| Cocok untuk | Nilai yang terus berubah (counter, akumulator) | Transformasi data (`String` → `i32`) |

#### e. Scope (ruang lingkup)
Variabel hanya hidup di dalam blok `{ }` tempat ia dibuat.

```rust
fn main() {
    let a = 1;
    {
        let b = 2;
        println!("{a} {b}"); // ✅
    }
    println!("{b}"); // ❌ error: cannot find value `b` in this scope
}
```

#### f. Deklarasi dulu, isi belakangan
```rust
let hasil;           // belum ada nilai
if true {
    hasil = "ya";
} else {
    hasil = "tidak";
}
println!("{hasil}"); // ✅ asal sudah pasti terisi sebelum dipakai
```
Memakai variabel yang belum diisi akan menghasilkan error. Rust **tidak punya** nilai `null` atau `undefined` bawaan.

#### g. Variabel yang tidak dipakai
```rust
let nama = "Apriana"; // ⚠️ warning: unused variable: `nama`
let _nama = "Apriana"; // ✅ awalan _ berarti "sengaja tidak dipakai"
let _ = hitung();      // ✅ buang hasilnya
```

#### h. Destructuring
Memecah tuple menjadi beberapa variabel sekaligus.
```rust
let (x, y) = (10, 20);
let (nama, _, kota) = ("Budi", 25, "Bali"); // _ = abaikan
```

### Aturan penamaan
| Jenis | Gaya | Contoh |
|---|---|---|
| Variabel & function | `snake_case` | `nama_lengkap`, `hitung_total` |
| Konstanta & static | `SCREAMING_SNAKE_CASE` | `MAKS_PERCOBAAN` |
| Struct, enum, trait | `PascalCase` | `TodoList` |

Nama boleh berisi huruf, angka, dan `_`, tapi tidak boleh diawali angka.

---

## 6. Konstanta

```rust
const MAKS_PERCOBAAN: u32 = 7;
const PI: f64 = 3.14159;
const DETIK_PER_JAM: u32 = 60 * 60; // boleh berupa ekspresi yang bisa dihitung saat compile

static NAMA_APP: &str = "Belajar Rust";
```

| | `let` | `let mut` | `const` | `static` |
|---|---|---|---|---|
| Bisa diubah | ❌ | ✅ | ❌ | ❌ (kecuali `unsafe`) |
| Tipe wajib ditulis | ❌ | ❌ | ✅ | ✅ |
| Bisa di luar function (global) | ❌ | ❌ | ✅ | ✅ |
| Nilai ditentukan | Saat program jalan | Saat program jalan | Saat compile | Saat compile |
| Shadowing | ✅ | ✅ | ❌ | ❌ |

> 💡 Gunakan `const` untuk angka "ajaib" supaya kode lebih mudah dibaca: `if percobaan > MAKS_PERCOBAAN` lebih jelas daripada `if percobaan > 7`.

---

## 7. Tipe Data

Rust adalah bahasa **statically typed**: tipe setiap variabel harus diketahui saat compile.

### 7.0 Peta Semua Tipe & Tipe Default

#### A. Tipe default (hasil tebakan compiler)
Kalau tipe tidak ditulis, compiler menebak tipenya dari bentuk nilai:

| Kamu menulis | Tipe default | Keterangan |
|---|---|---|
| `let a = 10;` | `i32` | Semua bilangan bulat tanpa suffix |
| `let a = -10;` | `i32` | |
| `let a = 0xff;` / `0b1010` / `0o17` | `i32` | Hex, biner, oktal tetap `i32` |
| `let a = 1_000_000;` | `i32` | `_` hanya pemisah agar mudah dibaca |
| `let a = 3.14;` | `f64` | Semua bilangan desimal |
| `let a = 1e3;` | `f64` | Notasi ilmiah (1000.0) |
| `let a = true;` | `bool` | |
| `let a = 'a';` | `char` | Kutip **satu** |
| `let a = "a";` | `&str` (`&'static str`) | Kutip **dua** |
| `let a = r"C:\x";` | `&str` | Raw string |
| `let a = b'a';` | `u8` | Byte (nilai ASCII) |
| `let a = b"ab";` | `&[u8; 2]` | Byte string |
| `let a = (1, 2.0, "x");` | `(i32, f64, &str)` | Tuple, tiap elemen ditebak sendiri |
| `let a = [1, 2, 3];` | `[i32; 3]` | Array |
| `let a = ();` | `()` | Unit, artinya "tidak ada nilai" |
| `let a = 1..5;` | `Range<i32>` | Range |
| `let a = 1..=5;` | `RangeInclusive<i32>` | Range inklusif |

#### B. Menentukan tipe sendiri
```rust
let a: u8 = 10;     // cara 1: type annotation
let b = 10u8;       // cara 2: suffix
let c = 10_u8;      // suffix dengan pemisah _ (lebih mudah dibaca)
let d = 2.5_f32;
```

#### C. ⚠️ Default hanya berlaku kalau tidak ada petunjuk lain
Compiler melihat **cara variabel dipakai**, bukan hanya baris saat variabel dibuat:
```rust
let x = 5;          // belum ada petunjuk...
let y: u64 = 10;
let z = x + y;      // ...x dipakai bersama u64, jadi x ikut menjadi u64 (bukan i32)
```
Ini juga berlaku saat nilai dikirim ke function atau disimpan ke variabel bertipe tertentu.

#### D. Daftar lengkap tipe primitif

**Integer (bilangan bulat)**

| Tipe | Ukuran | Minimum | Maksimum |
|---|---|---|---|
| `i8` | 1 byte | -128 | 127 |
| `i16` | 2 byte | -32.768 | 32.767 |
| **`i32`** ⭐ default | 4 byte | -2.147.483.648 | 2.147.483.647 |
| `i64` | 8 byte | -9.223.372.036.854.775.808 | 9.223.372.036.854.775.807 |
| `i128` | 16 byte | ≈ -1,7 × 10³⁸ | ≈ 1,7 × 10³⁸ |
| `isize` | 8 byte* | sama dengan `i64`* | sama dengan `i64`* |
| `u8` | 1 byte | 0 | 255 |
| `u16` | 2 byte | 0 | 65.535 |
| `u32` | 4 byte | 0 | 4.294.967.295 |
| `u64` | 8 byte | 0 | 18.446.744.073.709.551.615 |
| `u128` | 16 byte | 0 | ≈ 3,4 × 10³⁸ |
| `usize` | 8 byte* | 0 | sama dengan `u64`* |

\* `isize` dan `usize` mengikuti arsitektur CPU: 8 byte di komputer 64-bit, 4 byte di 32-bit. `usize` dipakai untuk **index dan panjang** (`.len()` menghasilkan `usize`).

> 💡 Huruf `i` = *integer* bertanda (bisa negatif), `u` = *unsigned* (tidak bisa negatif). Angkanya = jumlah bit.

**Float (bilangan desimal)**

| Tipe | Ukuran | Presisi | Keterangan |
|---|---|---|---|
| `f32` | 4 byte | ±6–7 digit | Lebih hemat memori |
| **`f64`** ⭐ default | 8 byte | ±15–16 digit | Lebih presisi, kecepatannya hampir sama di CPU modern |

**Lainnya**

| Tipe | Ukuran | Nilai | Contoh |
|---|---|---|---|
| `bool` | 1 byte | `true` / `false` | `let aktif = true;` |
| `char` | 4 byte | Satu karakter Unicode | `'A'`, `'🦀'`, `'ꦲ'` |
| `&str` | 16 byte (alamat + panjang) | Teks UTF-8 yang dipinjam | `"halo"` |
| `()` | 0 byte | Hanya `()` | Hasil function tanpa return |
| `!` (never) | – | Tidak punya nilai | Hasil dari `panic!()` atau `loop {}` tanpa `break` |

**Tipe gabungan (compound)**

| Tipe | Isi | Panjang | Contoh |
|---|---|---|---|
| Tuple `(T1, T2, ...)` | Tipe **boleh berbeda** | Tetap | `(1, "dua", true)` |
| Array `[T; N]` | Tipe **harus sama** | Tetap (`N`) | `[1, 2, 3]` → `[i32; 3]` |
| Slice `&[T]` | Potongan dari array atau Vec | Diketahui saat program jalan | `&arr[1..3]` |

#### E. Tipe dari standard library (dipelajari nanti)
Ini **bukan** tipe primitif, tapi akan sangat sering dipakai:

| Tipe | Kegunaan | Dipelajari di |
|---|---|---|
| `String` | Teks yang bisa diubah dan dimiliki sendiri | Fase 2 |
| `Vec<T>` | Daftar yang bisa bertambah atau berkurang | Fase 3 |
| `HashMap<K, V>` | Pasangan kunci–nilai (seperti dictionary) | Fase 3 |
| `Option<T>` | Nilai yang mungkin ada (`Some(x)`) atau tidak ada (`None`), pengganti `null` | Fase 3 |
| `Result<T, E>` | Hasil yang bisa sukses (`Ok(x)`) atau gagal (`Err(e)`) | Fase 3 |
| `Box<T>`, `Rc<T>`, `Arc<T>` | Smart pointer | Fase 4 |

#### F. Nilai default (`Default::default()`)
Beda dengan *tipe* default di atas. Ini adalah **nilai awal** setiap tipe kalau dibuat dengan `Default::default()`:

| Tipe | Nilai default |
|---|---|
| Semua integer | `0` |
| `f32`, `f64` | `0.0` |
| `bool` | `false` |
| `char` | `'\0'` (karakter kosong) |
| `&str`, `String` | `""` |
| `Vec<T>` | `[]` |
| `Option<T>` | `None` |
| Tuple | Default tiap elemen, contoh `(i32, bool)` → `(0, false)` |

```rust
let angka: i32 = Default::default();   // 0
let aktif: bool = Default::default();  // false
```

> ⚠️ Rust **tidak** otomatis memberi nilai default ke variabel. `let x: i32;` lalu langsung dipakai tetap error. Nilai default hanya dipakai kalau kamu memanggil `Default::default()` secara eksplisit.

#### G. Cara mengecek tipe variabel
1. **VS Code:** arahkan kursor ke nama variabel, rust-analyzer langsung menampilkan tipenya
2. **Sengaja bikin error:** `let _: () = variabel;` → pesan error menampilkan `found <tipe>`
3. **Saat program jalan:** `println!("{}", std::any::type_name_of_val(&variabel));`

### 7.1 Tipe Skalar (satu nilai)

#### Integer (bilangan bulat)
| Panjang | Bertanda (bisa negatif) | Tanpa tanda (≥ 0) |
|---|---|---|
| 8-bit | `i8` (-128 s/d 127) | `u8` (0 s/d 255) |
| 16-bit | `i16` | `u16` (0 s/d 65.535) |
| 32-bit | **`i32`** (default) | `u32` (0 s/d ~4,29 miliar) |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Ukuran CPU | `isize` | **`usize`** (untuk index & panjang) |

Cara menulis literal:
```rust
let juta = 1_000_000;    // _ sebagai pemisah ribuan agar mudah dibaca
let hex = 0xff;          // 255
let biner = 0b1010;      // 10
let oktal = 0o17;        // 15
let byte = b'A';         // 65 (u8)

println!("{}", i32::MAX); // 2147483647
println!("{}", u8::MIN);  // 0
```

⚠️ **Overflow:** angka melewati batas tipenya.
```rust
let x: u8 = 255;
let y = x + 1; // mode debug: PANIC (program berhenti). Mode release: jadi 0
```
Cara aman menangani overflow:
```rust
let x: u8 = 255;
x.checked_add(1);     // None (hasilnya Option)
x.wrapping_add(1);    // 0 (memutar kembali ke awal)
x.saturating_add(1);  // 255 (mentok di batas)
```

#### Float (desimal)
```rust
let a = 2.5;        // f64 (default, lebih presisi)
let b: f32 = 2.5;   // f32
```

#### Operasi aritmatika
```rust
let tambah = 5 + 3;     // 8
let kurang = 5 - 3;     // 2
let kali = 5 * 3;       // 15
let bagi = 7 / 2;       // 3   ← integer dibagi integer, desimalnya dibuang
let bagi_f = 7.0 / 2.0; // 3.5
let sisa = 7 % 2;       // 1   (modulo)
```

⚠️ Rust **tidak otomatis mengonversi** tipe. Konversi harus ditulis dengan `as`:
```rust
let a: i32 = 10;
let b: f64 = 2.5;
// let c = a * b;          // ❌ error: cannot multiply `i32` by `f64`
let c = a as f64 * b;      // ✅ 25.0
let d = 3.99_f64 as i32;   // 3 (dipotong, bukan dibulatkan)
```

#### Boolean
```rust
let aktif = true;
let lulus: bool = false;

// Operator: && (dan), || (atau), ! (tidak)
let boleh = aktif && !lulus;

// Perbandingan: ==, !=, <, >, <=, >=
let dewasa = 20 >= 17; // true
```

#### Char
Satu karakter Unicode (4 byte), ditulis dengan **kutip satu**.
```rust
let huruf = 'A';
let emoji = '🦀';
let aksara = 'ꦲ';   // aksara Jawa juga bisa

'a'.is_alphabetic(); // true
'7'.is_numeric();    // true
'a'.to_ascii_uppercase(); // 'A'
```

> ⚠️ `'A'` (char) ≠ `"A"` (string). Kutip satu untuk char, kutip dua untuk string.

### 7.2 Tipe Gabungan (Compound)

#### Tuple
Kumpulan beberapa nilai dengan **tipe boleh berbeda**. Panjangnya tetap.
```rust
let orang: (&str, i32, bool) = ("Apriana", 25, true);

println!("{}", orang.0); // Apriana (akses dengan index)
let (nama, umur, _) = orang; // destructuring

let kosong = (); // tuple kosong disebut "unit", artinya "tidak ada nilai"
```

#### Array
Kumpulan nilai dengan **tipe sama** dan **panjang tetap**.
```rust
let angka: [i32; 5] = [1, 2, 3, 4, 5];
let nol = [0; 3];        // [0, 0, 0]

println!("{}", angka[0]);     // 1
println!("{}", angka.len());  // 5
println!("{:?}", angka);      // [1, 2, 3, 4, 5]

angka[10]; // ❌ index di luar batas → panic (Rust tidak membiarkan akses memori sembarangan)
```

> 📝 Butuh daftar yang bisa **bertambah**? Pakai `Vec` (dibahas di Fase 3).

### 7.3 Sekilas: String
| | `&str` | `String` |
|---|---|---|
| Contoh | `"halo"` | `String::from("halo")` |
| Bisa diubah/ditambah | ❌ | ✅ (kalau `mut`) |
| Disimpan di | Tertanam di program / meminjam data lain | Heap (memori dinamis) |

```rust
let a: &str = "halo";
let mut b: String = String::from("halo");
b.push_str(" dunia");
let c = a.to_string(); // &str → String
```
Detailnya dipelajari di Fase 2 (Ownership).

### 7.4 Method untuk Setiap Tipe Data

**Method** adalah function yang "menempel" pada sebuah nilai, dipanggil dengan titik: `nilai.method()`.

#### ⚠️ Dua jebakan sebelum mulai

**1. Angka tanpa tipe tidak bisa langsung dipanggil method-nya**
```rust
2.pow(3);          // ❌ error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
2_i32.pow(3);      // ✅ 8, tipenya diberi suffix
let x: i32 = 2;
x.pow(3);          // ✅ 8, variabelnya sudah bertipe
```

**2. Tanda minus kalah prioritas dengan method**
```rust
-5_i32.abs();      // -5 ❗ dibaca sebagai -(5.abs())
(-5_i32).abs();    // 5  ✅ pakai kurung
```

> 💡 Cara melihat **semua** method sebuah tipe: ketik `nilai.` di VS Code, rust-analyzer akan menampilkan daftarnya. Dokumentasi lengkap ada di [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) (cari `i32`, `f64`, `char`, `str`, dan seterusnya).

---

#### 🔢 A. Integer (`i8`–`i128`, `u8`–`u128`, `isize`, `usize`)

**Matematika dasar**
| Method | Contoh | Hasil | Keterangan |
|---|---|---|---|
| `abs()` | `(-7_i32).abs()` | `7` | Nilai mutlak (hanya tipe `i`) |
| `pow(n)` | `2_i32.pow(10)` | `1024` | Pangkat |
| `isqrt()` | `17_u32.isqrt()` | `4` | Akar kuadrat, dibulatkan ke bawah |
| `signum()` | `(-7_i32).signum()` | `-1` | Tanda: `-1`, `0`, atau `1` |
| `is_positive()` | `7_i32.is_positive()` | `true` | |
| `is_negative()` | `(-7_i32).is_negative()` | `true` | |
| `min(b)` | `(-7_i32).min(3)` | `-7` | Nilai terkecil dari dua angka |
| `max(b)` | `(-7_i32).max(3)` | `3` | Nilai terbesar dari dua angka |
| `clamp(min, max)` | `15_i32.clamp(0, 10)` | `10` | Paksa nilai ke dalam rentang |
| `abs_diff(b)` | `10_u32.abs_diff(15)` | `5` | Selisih tanpa tanda |
| `rem_euclid(b)` | `(-7_i32).rem_euclid(3)` | `2` | Sisa bagi yang selalu positif (`-7 % 3` hasilnya `-1`) |
| `div_euclid(b)` | `(-7_i32).div_euclid(3)` | `-3` | Pembagian pasangan `rem_euclid` |
| `ilog10()` | `100_i32.ilog10()` | `2` | Logaritma basis 10, dibulatkan ke bawah |

**Menangani overflow** (`add` bisa diganti `sub`, `mul`, `div`, `pow`)
| Method | Contoh | Hasil | Keterangan |
|---|---|---|---|
| `checked_add(b)` | `255_u8.checked_add(1)` | `None` | `Some(hasil)` kalau aman, `None` kalau overflow |
| `checked_div(b)` | `10_i32.checked_div(0)` | `None` | Aman dari pembagian dengan nol |
| `wrapping_add(b)` | `255_u8.wrapping_add(1)` | `0` | Memutar kembali ke awal |
| `saturating_add(b)` | `255_u8.saturating_add(1)` | `255` | Mentok di batas maksimum |
| `saturating_sub(b)` | `0_u8.saturating_sub(1)` | `0` | Mentok di batas minimum |
| `overflowing_add(b)` | `255_u8.overflowing_add(1)` | `(0, true)` | Hasil + penanda apakah terjadi overflow |

**Bit & bilangan biner**
| Method | Contoh | Hasil |
|---|---|---|
| `count_ones()` | `0b1011_u8.count_ones()` | `3` |
| `leading_zeros()` | `1_u8.leading_zeros()` | `7` |
| `is_power_of_two()` | `8_u32.is_power_of_two()` | `true` (hanya tipe `u`) |
| `next_power_of_two()` | `10_u32.next_power_of_two()` | `16` (hanya tipe `u`) |

**Konstanta & konversi**
| Kode | Hasil |
|---|---|
| `i32::MAX` / `i32::MIN` | `2147483647` / `-2147483648` |
| `i32::BITS` | `32` |
| `42.to_string()` | `"42"` |
| `i32::from_str_radix("ff", 16)` | `Ok(255)` (dari teks basis 16) |
| `format!("{:b} {:x} {:o} {:X}", 10, 255, 8, 255)` | `"1010 ff 10 FF"` (biner, hex, oktal) |
| `10 as f64` | `10.0` |

---

#### 🌊 B. Float (`f32`, `f64`)

**Pembulatan**
| Method | `3.7` | `-3.7` | Keterangan |
|---|---|---|---|
| `floor()` | `3.0` | `-4.0` | Bulatkan ke bawah |
| `ceil()` | `4.0` | `-3.0` | Bulatkan ke atas |
| `round()` | `4.0` | `-4.0` | Bulatkan ke terdekat (`2.5` → `3.0`) |
| `trunc()` | `3.0` | `-3.0` | Buang bagian desimal |
| `fract()` | `0.7` | `-0.7` | Ambil bagian desimal saja |
| `abs()` | `3.7` | `3.7` | Nilai mutlak |

**Matematika**
| Method | Contoh | Hasil |
|---|---|---|
| `sqrt()` | `16.0_f64.sqrt()` | `4.0` |
| `cbrt()` | `8.0_f64.cbrt()` | `2.0` (akar pangkat tiga) |
| `powi(n)` | `2.0_f64.powi(3)` | `8.0` (pangkat bilangan bulat, lebih cepat) |
| `powf(x)` | `4.0_f64.powf(0.5)` | `2.0` (pangkat desimal) |
| `exp()` | `0.0_f64.exp()` | `1.0` (eˣ) |
| `ln()` | `E.ln()` | `1.0` (logaritma natural) |
| `log10()` | `100.0_f64.log10()` | `2.0` |
| `log2()` | `8.0_f64.log2()` | `3.0` |
| `hypot(b)` | `3.0_f64.hypot(4.0)` | `5.0` (sisi miring segitiga, √(a²+b²)) |
| `min(b)` / `max(b)` | `3.7_f64.min(1.0)` | `1.0` |
| `clamp(min, max)` | `15.5_f64.clamp(0.0, 10.0)` | `10.0` |

**Trigonometri** (dalam radian)
| Method | Contoh | Hasil |
|---|---|---|
| `sin()`, `cos()`, `tan()` | `0.0_f64.cos()` | `1.0` |
| `asin()`, `acos()`, `atan()`, `atan2(x)` | | Kebalikan dari fungsi trigonometri |
| `to_radians()` | `180.0_f64.to_radians()` | `3.14159...` (π) |
| `to_degrees()` | `PI.to_degrees()` | `180.0` |

**Pengecekan & nilai khusus**
| Kode | Hasil | Keterangan |
|---|---|---|
| `f64::NAN.is_nan()` | `true` | NaN = *Not a Number*, misalnya hasil `0.0 / 0.0` |
| `f64::INFINITY.is_finite()` | `false` | |
| `1.0 / 0.0` | `inf` | Pembagian float dengan nol **tidak** panic |
| `(-1.0_f64).is_sign_negative()` | `true` | |
| `std::f64::consts::PI`, `E` | `3.14159...`, `2.71828...` | Konstanta matematika |
| `f64::MAX`, `f64::MIN`, `f64::EPSILON` | | Batas & selisih terkecil |
| `3.7 as i32` | `3` | Dipotong, bukan dibulatkan |
| `format!("{:.2}", 3.14159)` | `"3.14"` | |

> ⚠️ **Jangan bandingkan float dengan `==`**. `0.1 + 0.2 == 0.3` hasilnya `false`, karena komputer menyimpan desimal secara tidak persis. Pakai selisih kecil:
> ```rust
> ((0.1_f64 + 0.2) - 0.3).abs() < f64::EPSILON   // true
> ```

---

#### ✅ C. Boolean (`bool`)

| Kode | Hasil | Keterangan |
|---|---|---|
| `!true` | `false` | NOT |
| `a && b` / `a \|\| b` | | AND / OR |
| `true ^ true` | `false` | XOR (benar kalau hanya salah satu yang `true`) |
| `true.then(\|\| 5)` | `Some(5)` | Jalankan closure kalau `true`, hasilnya `Option` |
| `false.then_some(5)` | `None` | Seperti `then`, tapi langsung dengan nilai |
| `true as i32` | `1` | `false as i32` → `0` |
| `true.to_string()` | `"true"` | |
| `"true".parse::<bool>()` | `Ok(true)` | |

---

#### 🔤 D. Char (`char`)

**Pengecekan**
| Method | `true` untuk | Keterangan |
|---|---|---|
| `is_alphabetic()` | `'a'`, `'é'`, `'ꦲ'` | Huruf (semua bahasa) |
| `is_numeric()` | `'7'` | Angka (semua sistem angka Unicode) |
| `is_alphanumeric()` | `'x'`, `'7'` | Huruf atau angka |
| `is_whitespace()` | `' '`, `'\n'`, `'\t'` | Spasi, tab, enter |
| `is_uppercase()` / `is_lowercase()` | `'A'` / `'a'` | |
| `is_ascii()` | `'a'` (bukan `'é'`) | Karakter ASCII (0–127) |
| `is_ascii_digit()` | `'0'`–`'9'` | Hanya angka 0–9 |
| `is_ascii_punctuation()` | `'!'`, `'.'`, `','` | Tanda baca |
| `eq_ignore_ascii_case(&b)` | `'a'` vs `'A'` | Bandingkan tanpa peduli huruf besar/kecil |

**Konversi**
| Kode | Hasil | Keterangan |
|---|---|---|
| `'a'.to_ascii_uppercase()` | `'A'` | Hasilnya `char` |
| `'A'.to_ascii_lowercase()` | `'a'` | |
| `'ß'.to_uppercase().to_string()` | `"SS"` | Versi Unicode bisa menghasilkan lebih dari 1 karakter |
| `'7'.to_digit(10)` | `Some(7)` | Char → angka (basis 10) |
| `'f'.to_digit(16)` | `Some(15)` | Basis 16 |
| `char::from_digit(7, 10)` | `Some('7')` | Angka → char |
| `'A' as u32` | `65` | Kode Unicode |
| `char::from_u32(65)` | `Some('A')` | Kode → char |
| `(b'a' + 1) as char` | `'b'` | Huruf berikutnya |
| `'🦀'.len_utf8()` | `4` | Jumlah byte jika disimpan di string |
| `'a'.to_string()` | `"a"` | Char → String |
| `('a'..='z').contains(&'m')` | `true` | Cek apakah dalam rentang |

---

#### 🧵 E. String (`&str` dan `String`)

Semua method di tabel E1–E5 bisa dipakai pada `&str` **maupun** `String`. Tabel E6 khusus `String`.

**E1. Info & pengecekan**
| Method | Contoh (`s = "Halo Rust"`) | Hasil |
|---|---|---|
| `len()` | `s.len()` | `9` (jumlah **byte**) |
| `chars().count()` | `s.chars().count()` | `9` (jumlah **karakter**) |
| `is_empty()` | `s.is_empty()` | `false` |
| `contains(x)` | `s.contains("Rust")` | `true` |
| `starts_with(x)` | `s.starts_with("Halo")` | `true` |
| `ends_with(x)` | `s.ends_with("st")` | `true` |
| `find(x)` | `s.find('R')` | `Some(5)` (posisi byte) |
| `rfind(x)` | `"a-b-a".rfind('a')` | `Some(4)` (cari dari belakang) |
| `matches(x).count()` | `"halo".matches('a').count()` | `1` |
| `eq_ignore_ascii_case(x)` | `"RUST".eq_ignore_ascii_case("rust")` | `true` |
| `chars().all(f)` | `"123".chars().all(\|c\| c.is_ascii_digit())` | `true` |
| `chars().any(f)` | `"a1".chars().any(\|c\| c.is_numeric())` | `true` |
| `==`, `<`, `>` | `"apel" < "jeruk"` | `true` (urutan abjad) |

**E2. Mengubah bentuk** (menghasilkan nilai baru, aslinya tidak berubah)
| Method | Contoh | Hasil |
|---|---|---|
| `to_uppercase()` | `s.to_uppercase()` | `"HALO RUST"` |
| `to_lowercase()` | `s.to_lowercase()` | `"halo rust"` |
| `trim()` | `"  x  ".trim()` | `"x"` |
| `trim_start()` / `trim_end()` | `"  x  ".trim_start()` | `"x  "` |
| `trim_matches(c)` | `"xxhaloxx".trim_matches('x')` | `"halo"` |
| `strip_prefix(x)` | `"Rp100".strip_prefix("Rp")` | `Some("100")` |
| `strip_suffix(x)` | `"foto.jpg".strip_suffix(".jpg")` | `Some("foto")` |
| `replace(a, b)` | `s.replace("Rust", "Dunia")` | `"Halo Dunia"` |
| `replacen(a, b, n)` | `"aaa".replacen("a", "b", 2)` | `"bba"` |
| `repeat(n)` | `"ab".repeat(3)` | `"ababab"` |

**E3. Memecah**
| Method | Contoh | Hasil |
|---|---|---|
| `split(x)` | `"a,b,c".split(',')` | `"a"`, `"b"`, `"c"` |
| `splitn(n, x)` | `"a,b,c".splitn(2, ',')` | `"a"`, `"b,c"` |
| `split_once(x)` | `"k=v".split_once('=')` | `Some(("k", "v"))` |
| `split_whitespace()` | `" a  b ".split_whitespace()` | `"a"`, `"b"` |
| `lines()` | `"a\nb".lines()` | `"a"`, `"b"` |

> Hasil `split` berupa **iterator**. Untuk dijadikan daftar: `.collect::<Vec<_>>()`.

**E4. Per karakter / byte**
| Method | Contoh | Hasil |
|---|---|---|
| `chars()` | `"abc".chars()` | `'a'`, `'b'`, `'c'` |
| `chars().nth(n)` | `"abc".chars().nth(1)` | `Some('b')` |
| `chars().rev().collect::<String>()` | `"abc"` | `"cba"` |
| `char_indices()` | `"Halo".char_indices()` | `(0,'H')`, `(1,'a')`, ... |
| `bytes()` | `"abc".bytes()` | `97`, `98`, `99` |
| `as_bytes()` | `"abc".as_bytes()` | `[97, 98, 99]` |
| `&s[a..b]` | `&s[0..4]` | `"Halo"` (panic kalau memotong di tengah karakter) |
| `get(a..b)` | `s.get(0..100)` | `None` (versi aman) |

**E5. Konversi & menggabungkan**
| Kode | Hasil |
|---|---|
| `"42".parse::<i32>()` | `Ok(42)` |
| `"abc".parse::<i32>()` | `Err(...)` |
| `"halo".to_string()` / `String::from("halo")` | `String` |
| `["a", "b"].join("-")` | `"a-b"` |
| `["a", "b"].concat()` | `"ab"` |
| `format!("{}-{}", a, b)` | `String` baru |

**E6. Khusus `String`** (mengubah isi, variabel harus `mut`)
| Method | Sebelum → Sesudah | Keterangan |
|---|---|---|
| `push(c)` | `"Halo"` → `"Halo!"` | Tambah 1 char |
| `push_str(s)` | `"Halo!"` → `"Halo! Rust"` | Tambah teks |
| `+= "x"` | `"ab"` → `"abx"` | Sama dengan `push_str` |
| `insert(i, c)` | `"Halo"` → `">Halo"` (i=0) | Sisipkan char di posisi byte `i` |
| `insert_str(i, s)` | | Sisipkan teks |
| `pop()` | `"Rust"` → `"Rus"`, menghasilkan `Some('t')` | Hapus char terakhir |
| `remove(i)` | `">Halo"` → `"Halo"`, menghasilkan `'>'` | Hapus char di posisi `i` |
| `truncate(n)` | `"Halo Rust"` → `"Halo"` (n=4) | Potong menjadi `n` byte |
| `retain(f)` | `"a b c"` → `"abc"` (buang spasi) | Simpan char yang memenuhi syarat |
| `replace_range(r, s)` | `">Halo"` → `"<Halo"` (`0..1`, `"<"`) | Ganti bagian tertentu |
| `drain(r)` | `"abc"` → `"bc"`, mengambil `"a"` (`..1`) | Ambil dan hapus sebagian |
| `clear()` | `"abc"` → `""` | Kosongkan |
| `make_ascii_uppercase()` | `"xyz"` → `"XYZ"` | Ubah langsung, tanpa membuat String baru |
| `extend(iter)` | `"x"` → `"xyz"` (`['y', 'z']`) | Tambah banyak char |
| `as_str()` | | `String` → `&str` |
| `clone()` | | Salin menjadi String baru |
| `capacity()` | | Kapasitas memori yang sudah disiapkan |
| `String::with_capacity(n)` | | Buat String kosong dengan kapasitas `n` |
| `String::from_utf8(bytes)` | `[104, 105]` → `Ok("hi")` | Dari byte → String |

---

#### 📦 F. Tuple

Tuple **tidak punya method khusus**. Yang bisa dilakukan:

| Operasi | Contoh | Hasil |
|---|---|---|
| Akses dengan index | `(1, "dua").0` | `1` |
| Ubah elemen (kalau `mut`) | `t.0 = 10;` | |
| Destructuring | `let (x, y) = (1, 2);` | `x = 1`, `y = 2` |
| Bandingkan | `(1, 2) == (1, 2)` | `true` |
| Urutkan (elemen pertama dulu, lalu kedua, dst.) | `(1, 2) < (1, 3)` | `true` |
| Cetak | `println!("{:?}", (1, "dua"))` | `(1, "dua")` |
| `clone()`, `Default::default()` | | Kalau semua elemennya mendukung |

---

#### 🗂️ G. Array (`[T; N]`)

Banyak method berikut juga berlaku untuk slice `&[T]` dan `Vec<T>` (Fase 3).

**Info & akses** (`arr = [5, 3, 8, 1, 9]`)
| Method | Hasil | Keterangan |
|---|---|---|
| `arr.len()` | `5` | |
| `arr.is_empty()` | `false` | |
| `arr[0]` | `5` | Panic kalau index di luar batas |
| `arr.get(10)` | `None` | Versi aman dari `arr[i]` |
| `arr.first()` / `arr.last()` | `Some(&5)` / `Some(&9)` | |
| `arr.contains(&8)` | `true` | |
| `arr.starts_with(&[5, 3])` | `true` | |
| `&arr[1..3]` | `[3, 8]` | Slice (potongan) |
| `arr.split_at(2)` | `([5, 3], [8, 1, 9])` | Bagi dua |

**Lewat iterator** (`.iter()`)
| Kode | Hasil |
|---|---|
| `arr.iter().sum::<i32>()` | `26` |
| `arr.iter().max()` / `.min()` | `Some(&9)` / `Some(&1)` |
| `arr.iter().count()` | `5` |
| `arr.iter().position(\|&x\| x == 8)` | `Some(2)` (index pertama yang cocok) |
| `arr.iter().filter(\|&&x\| x > 4).count()` | `3` |
| `arr.iter().rev()` | `9, 1, 8, 3, 5` |
| `arr.iter().enumerate()` | `(0, 5), (1, 3), ...` |
| `arr.map(\|x\| x * 2)` | `[10, 6, 16, 2, 18]` (array baru) |

**Mengubah isi** (array harus `mut`)
| Method | Hasil dari `[5, 3, 8, 1, 9]` |
|---|---|
| `sort()` | `[1, 3, 5, 8, 9]` |
| `sort_unstable()` | Sama, lebih cepat, urutan elemen yang nilainya sama tidak dijamin |
| `sort_by(\|a, b\| b.cmp(a))` | `[9, 8, 5, 3, 1]` (urutan sendiri: menurun) |
| `reverse()` | `[9, 1, 8, 3, 5]` |
| `swap(0, 4)` | `[9, 3, 8, 1, 5]` |
| `rotate_left(1)` | `[3, 8, 1, 9, 5]` |
| `fill(0)` | `[0, 0, 0, 0, 0]` |

**Lainnya**
| Method | Contoh | Hasil |
|---|---|---|
| `binary_search(&x)` | `[1, 3, 5, 8, 9].binary_search(&8)` | `Ok(3)` (array harus sudah urut) |
| `windows(n)` | `[1, 2, 3].windows(2)` | `[1, 2]`, `[2, 3]` |
| `chunks(n)` | `[1, 2, 3].chunks(2)` | `[1, 2]`, `[3]` |
| `concat()` | `[[1, 2], [3, 4]].concat()` | `[1, 2, 3, 4]` |
| `join(x)` | `["a", "b"].join("-")` | `"a-b"` |
| `to_vec()` | `[1, 2].to_vec()` | `vec![1, 2]` |

---

#### 🔁 H. Konversi Antar Tipe (ringkasan)

| Dari → Ke | Cara | Contoh |
|---|---|---|
| Angka → angka | `as` | `10_i32 as f64` → `10.0` |
| Angka → angka (aman) | `try_from` / `try_into` | `u8::try_from(300_i32)` → `Err` |
| Angka → angka (pasti aman) | `from` / `into` | `i64::from(10_i32)` → `10` |
| Apa pun → `String` | `to_string()` | `42.to_string()`, `'a'.to_string()` |
| `&str` → angka / bool | `parse::<T>()` | `"42".parse::<i32>()` → `Ok(42)` |
| `&str` → `String` | `to_string()`, `String::from()`, `into()` | |
| `String` → `&str` | `&s` atau `as_str()` | |
| `char` → angka | `to_digit(10)` atau `as u32` | |
| angka → `char` | `char::from_digit()` atau `char::from_u32()` | |
| `bool` → angka | `as` | `true as i32` → `1` |

---

## 8. Function

### Definisi
**Function** adalah blok kode bernama yang bisa dipanggil berulang kali.

```rust
fn sapa() {
    println!("Halo!");
}

fn tambah(a: i32, b: i32) -> i32 {
    a + b // tanpa `;` → menjadi nilai return
}

fn main() {
    sapa();
    let hasil = tambah(2, 3);
    println!("{hasil}"); // 5
}
```

### Aturan
- Tipe **parameter wajib ditulis**: `fn f(x: i32)` ✅, `fn f(x)` ❌
- Tipe return ditulis setelah `->`
- Function tanpa `->` mengembalikan `()` (unit, artinya tidak ada nilai)
- Urutan penulisan function bebas. Boleh memanggil function yang ditulis di bawahnya
- Rust **tidak punya** parameter default dan function overloading

### Return lebih awal
```rust
fn bagi(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 0; // keluar lebih awal dengan `return`
    }
    a / b
}
```

### Mengembalikan beberapa nilai sekaligus (pakai tuple)
```rust
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

let (kecil, besar) = min_max(9, 3); // kecil = 3, besar = 9
```

---

## 9. Statement vs Expression

Konsep ini **sangat penting** di Rust.

| | Statement | Expression |
|---|---|---|
| Arti | Melakukan sesuatu | Menghasilkan nilai |
| Diakhiri `;` | Ya | Tidak (kalau `;` ditambahkan, ia jadi statement) |
| Contoh | `let x = 5;` | `5`, `a + b`, `if ... {}`, blok `{}` |

```rust
let y = {
    let x = 3;
    x + 1      // ← expression, nilai blok ini = 4
};             // y = 4

fn dua() -> i32 {
    2;  // ❌ error: titik koma mengubahnya jadi statement, function mengembalikan () bukan i32
}
```

> 💡 Error "mismatched types: expected `i32`, found `()`" hampir selalu berarti ada **`;` yang tidak sengaja** di baris terakhir.

---

## 10. Kontrol Alur

### 10.1 `if` / `else if` / `else`
```rust
let nilai = 85;

if nilai >= 90 {
    println!("A");
} else if nilai >= 80 {
    println!("B");
} else {
    println!("C");
}
```

- Kondisi **harus `bool`**. `if 1 { }` ❌ (beda dengan JavaScript/PHP/C)
- Tidak perlu kurung `()` di kondisi

`if` adalah **expression**, jadi bisa menghasilkan nilai (pengganti ternary `? :`):
```rust
let status = if nilai >= 75 { "lulus" } else { "remedial" };
// Kedua cabang harus menghasilkan tipe yang sama
```

### 10.2 `loop`: perulangan tanpa henti
```rust
let mut hitung = 0;
let hasil = loop {
    hitung += 1;
    if hitung == 10 {
        break hitung * 2; // loop bisa mengembalikan nilai lewat break
    }
};
println!("{hasil}"); // 20
```

### 10.3 `while`: selama kondisi benar
```rust
let mut n = 3;
while n > 0 {
    println!("{n}...");
    n -= 1;
}
println!("Meluncur! 🚀");
```

### 10.4 `for`: perulangan pada range atau koleksi
```rust
for i in 1..5 { }        // 1, 2, 3, 4      (5 tidak ikut)
for i in 1..=5 { }       // 1, 2, 3, 4, 5   (5 ikut)
for i in (1..=5).rev() { } // 5, 4, 3, 2, 1
for i in (0..10).step_by(2) { } // 0, 2, 4, 6, 8

let buah = ["apel", "jeruk", "mangga"];
for b in buah { }                              // setiap elemen
for (i, b) in buah.iter().enumerate() { }      // index + elemen
```

> 💡 `for` adalah cara paling umum dan paling aman untuk perulangan di Rust, karena tidak mungkin terjadi index di luar batas.

### 10.5 `break` & `continue`
```rust
for i in 1..=10 {
    if i % 2 == 0 { continue; } // lewati angka genap
    if i > 7 { break; }          // berhenti total
    println!("{i}");             // 1 3 5 7
}
```

**Loop label** (untuk keluar dari loop bersarang):
```rust
'luar: for x in 0..5 {
    for y in 0..5 {
        if x + y == 6 {
            break 'luar; // keluar dari loop paling luar
        }
    }
}
```

### 10.6 Sekilas: `match`
Seperti `switch`, tapi jauh lebih kuat. Semua kemungkinan **wajib** ditangani.
```rust
let angka = 3;
match angka {
    1 => println!("satu"),
    2 | 3 => println!("dua atau tiga"),
    4..=9 => println!("empat sampai sembilan"),
    _ => println!("lainnya"), // _ = semua sisanya
}

// match juga expression
let teks = match angka % 2 {
    0 => "genap",
    _ => "ganjil",
};
```

---

## 11. Membaca Pesan Error

Compiler Rust terkenal sangat membantu. Biasakan **membaca error sampai habis**.

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

| Bagian | Arti |
|---|---|
| `error[E0384]` | Kode error. Jalankan `rustc --explain E0384` untuk penjelasan lengkap |
| `--> src/main.rs:3:5` | File : baris : kolom |
| `help:` | **Saran perbaikan**, sering bisa langsung dipakai |

### Error yang sering muncul di hari pertama
| Error | Penyebab | Solusi |
|---|---|---|
| `cannot assign twice to immutable variable` | Mengubah variabel tanpa `mut` | Tambahkan `mut` |
| `mismatched types` | Tipe tidak cocok | Cek tipe atau konversi dengan `as` |
| `expected i32, found ()` | Ada `;` di baris terakhir function | Hapus `;` |
| `cannot find value in this scope` | Variabel dipakai di luar bloknya, atau salah ketik | Cek scope dan ejaan |
| `used binding isn't initialized` | Variabel dipakai sebelum diisi | Isi dulu sebelum dipakai |
| `warning: unused variable` | Variabel dibuat tapi tidak dipakai | Pakai variabelnya, atau beri awalan `_` |

---

## 12. Rangkuman

- [ ] Cargo: `new`, `check`, `build`, `run`, `fmt`, `clippy`
- [ ] Program mulai dari `fn main()`
- [ ] `println!` adalah macro; format dengan `{}`, `{:?}`, `{nama}`
- [ ] Variabel **immutable by default**; pakai `mut` agar bisa diubah
- [ ] Shadowing = `let` lagi dengan nama sama, boleh ganti tipe
- [ ] `const` wajib ditulis tipenya dan bisa global
- [ ] Integer default `i32`, float default `f64`, index memakai `usize`
- [ ] Tidak ada konversi tipe otomatis; pakai `as`
- [ ] `char` pakai `'...'`, string pakai `"..."`
- [ ] Tuple bisa berisi tipe berbeda; array tipenya sama dan panjangnya tetap
- [ ] Parameter function **wajib** ditulis tipenya
- [ ] Baris terakhir tanpa `;` = nilai return
- [ ] `if`, `match`, `loop`, dan blok `{}` adalah **expression**
- [ ] Kondisi `if` harus `bool`
- [ ] `1..5` tidak termasuk 5, `1..=5` termasuk 5

---

## 13. Challenge

### 🟢 Challenge 0: Halo, Rustacean
1. Ubah `src/main.rs` agar mencetak: `Halo, saya <nama kamu> dan saya belajar Rust!` (pakai variabel `nama`)
2. Jalankan `cargo run`, `cargo fmt`, dan `cargo clippy`. Pastikan **tidak ada warning**
3. Jawab dengan kata-katamu sendiri: apa beda `cargo check`, `cargo build`, dan `cargo run`?

**Jawaban:**
> _tulis di sini_

### 🟢 Challenge 1.1: FizzBuzz
Cetak angka 1 sampai 100:
- Kelipatan 3 → `Fizz`
- Kelipatan 5 → `Buzz`
- Kelipatan 3 dan 5 → `FizzBuzz`
- Selain itu → angkanya

**⭐ Bonus:** buat versi kedua memakai `match (n % 3, n % 5)`.

### 🧪 Eksperimen Wajib
Coba sengaja buat error berikut, jalankan `cargo check`, lalu **baca pesannya**:
1. Ubah variabel tanpa `mut`
2. Tambahkan `;` di baris terakhir function yang punya return value
3. `let x: u8 = 256;`
4. `if 1 { }`
5. Kalikan `i32` dengan `f64`

### ✍️ Mini Quiz
1. Apa beda `let mut x = 5; x = 6;` dengan `let x = 5; let x = 6;`?
2. Berapa hasil `7 / 2` dan `7.0 / 2.0`? Kenapa berbeda?
3. Kenapa kode `let a = if true { 1 } else { "satu" };` error?
4. Apa isi variabel `y`?
   ```rust
   let y = { let x = 2; x * 3 };
   ```
5. Berapa kali `println!` berikut dijalankan?
   ```rust
   for i in 0..3 { println!("{i}"); }
   ```

**Jawaban:**
> 1. _..._
> 2. _..._
> 3. _..._
> 4. _..._
> 5. _..._
