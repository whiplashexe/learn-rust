# 🦀 Roadmap Rust → REST API (dengan Challenge)

## Cara pakai

Setiap fase berisi **materi** dan **3 challenge**:

| Level | Arti |
|---|---|
| 🟢 Pemanasan | Latihan langsung dari materi |
| 🟡 Utama | Menggabungkan beberapa konsep |
| 🔴 Boss | Wajib lulus sebelum pindah ke fase berikutnya |

**Aturan umum (berlaku di semua challenge):**
- Buat tiap challenge sebagai project terpisah: `cargo new challenges/fase-1/fizzbuzz`
- Lulus jika `cargo clippy` **tanpa warning** dan kode sudah di-`cargo fmt`
- Mulai Fase 3: **dilarang `unwrap()` / `expect()`** di kode utama (boleh di test)
- Jangan langsung lihat solusi di internet. Kalau stuck lebih dari 30 menit, baca bagian 💡 Hint

---

## Fase 0 — Persiapan (1 hari)

**Materi:** install `rustup`, VS Code + rust-analyzer, `cargo new/run/check/fmt/clippy`.

### 🟢 Challenge 0: Halo, Rustacean
- Buat project `halo`, ubah outputnya menjadi `Halo, saya <nama kamu> dan saya belajar Rust!`
- Jalankan `rustc --version`, `cargo fmt`, dan `cargo clippy`.

**Lulus jika:** program jalan dan kamu paham beda `cargo check` vs `cargo build` vs `cargo run`.

---

## Fase 1 — Dasar Bahasa (minggu 1)

**Materi:** variabel, `mut`, shadowing, tipe data, function, `if`, `loop`, `while`, `for`, `String` vs `&str`.
📖 Rust Book bab 1–3

### 🟢 Challenge 1.1: FizzBuzz
Cetak angka 1–100. Kelipatan 3 → `Fizz`, kelipatan 5 → `Buzz`, kelipatan keduanya → `FizzBuzz`.

**Bonus:** buat versi kedua dengan `match` pada tuple `(n % 3, n % 5)`.

### 🟡 Challenge 1.2: Konverter Suhu Interaktif
```
Masukkan suhu: 100
Satuan (C/F): C
100°C = 212°F
```
- Baca input dari keyboard
- Input tidak valid (misal `abc`) **tidak boleh bikin program crash**. Minta ulang input.

💡 Hint: `std::io::stdin().read_line(&mut s)`, `s.trim().parse::<f64>()`, lalu `match` hasilnya.

### 🔴 Boss 1: Tebak Angka
- Program memilih angka acak 1–100 (crate `rand`, `cargo add rand`)
- User menebak, program menjawab "terlalu besar" / "terlalu kecil"
- Maksimal **7 percobaan**, tampilkan sisa percobaan
- Setelah selesai, tanya `Main lagi? (y/n)`

**Lulus jika:** semua fitur jalan dan input ngawur tidak membuat crash.

---

## Fase 2 — Ownership & Borrowing (minggu 2)

**Materi:** ownership, move, clone, `&` dan `&mut`, slices, dasar lifetime.
📖 Rust Book bab 4

### 🟢 Challenge 2.1: Perbaiki Compiler Error
Ketiga kode di bawah **tidak bisa di-compile**. Perbaiki **tanpa memakai `.clone()`**, lalu tulis komentar yang menjelaskan *kenapa* error terjadi.

```rust
// A
fn main() {
    let s = String::from("halo");
    let t = s;
    println!("{s} {t}");
}
```
```rust
// B — output harus: "halo dunia"
fn tambah(s: String) {
    s.push_str(" dunia");
}
fn main() {
    let s = String::from("halo");
    tambah(s);
    println!("{s}");
}
```
```rust
// C — output harus: "1" lalu "[1, 2, 3, 4]"
fn main() {
    let mut v = vec![1, 2, 3];
    let pertama = &v[0];
    v.push(4);
    println!("{pertama}");
    println!("{v:?}");
}
```

### 🟡 Challenge 2.2: Fungsi Teks
Implementasikan fungsi berikut. **Semua parameter harus `&str`**, bukan `String`.
```rust
fn kata_pertama(s: &str) -> &str        // "halo dunia" -> "halo"
fn hitung_vokal(s: &str) -> usize       // "Rust Keren" -> 3
fn balik_kata(s: &str) -> String        // "saya suka rust" -> "rust suka saya"
fn kata_terpanjang(s: &str) -> &str     // "aku belajar rust" -> "belajar"
```
Input kosong `""` harus ditangani dengan benar (tidak panic).

### 🔴 Boss 2: Statistik Teks
- Baca file `input.txt` (`std::fs::read_to_string`)
- Tampilkan: jumlah baris, jumlah kata, jumlah karakter (tanpa spasi), kata terpanjang, rata-rata panjang kata
- Semua fungsi menerima `&str`. **Dilarang `.clone()`**

**Lulus jika:** hasil benar untuk file teks apa pun, termasuk file kosong.

---

## Fase 3 — Struct, Enum & Error Handling (minggu 3)

**Materi:** `struct`, `impl`, `enum`, `match`, `Option`, `Result`, operator `?`, `Vec`, `HashMap`, module.
📖 Rust Book bab 5–9

### 🟢 Challenge 3.1: Persegi Panjang
```rust
struct PersegiPanjang { lebar: u32, tinggi: u32 }
```
Buat method `luas()`, `keliling()`, `bisa_menampung(&self, lain: &PersegiPanjang) -> bool`, dan associated function `persegi(sisi: u32) -> Self`. Tambahkan `#[derive(Debug)]` lalu print dengan `{:#?}`.

### 🟡 Challenge 3.2: Parser Perintah
```rust
enum Perintah { Tambah(String), Selesai(u32), Hapus(u32), Daftar, Keluar }
enum ParseError { Kosong, PerintahTakDikenal(String), IdTidakValid }

fn parse(input: &str) -> Result<Perintah, ParseError>
```
| Input | Hasil |
|---|---|
| `tambah beli susu` | `Ok(Tambah("beli susu"))` |
| `selesai 2` | `Ok(Selesai(2))` |
| `hapus abc` | `Err(IdTidakValid)` |
| `terbang` | `Err(PerintahTakDikenal("terbang"))` |

### 🔴 Boss 3: Todo CLI (in-memory)
Gabungkan 3.2 menjadi aplikasi REPL:
```
> tambah belajar ownership
✅ Todo #1 ditambahkan
> daftar
[ ] #1 belajar ownership
> selesai 1
> hapus 99
❌ Todo #99 tidak ditemukan
```
- `struct Todo`, `struct TodoList` dengan method yang mengembalikan `Result`
- Pisahkan kode ke module: `main.rs`, `todo.rs`, `perintah.rs`
- **Nol `unwrap()`**. Semua error ditampilkan dengan pesan ramah
- Bonus: perintah `statistik` menampilkan 5 kata paling sering dipakai di semua judul todo (pakai `HashMap`)

---

## Fase 4 — Generics, Traits, Closure, Iterator & Testing (minggu 4)

**Materi:** generics, trait, `impl Trait`, `dyn Trait`, closure, iterator, `Box`/`Rc`/`Arc`/`Mutex`, unit test.
📖 Rust Book bab 10, 11, 13, 15, 16

### 🟢 Challenge 4.1: Iterator Drill
Dari `let v = vec![5, 12, 7, 3, 20, 8, 15];`, kerjakan **tanpa `for` loop**, hanya dengan rantai iterator:
1. Jumlah semua bilangan genap → `40`
2. Kuadrat dari bilangan ganjil → `[25, 49, 9, 225]`
3. Nilai maksimum → `Some(20)`
4. 3 angka terbesar, urut menurun → `[20, 15, 12]`
5. `"belajar rust itu seru"` → `"Belajar Rust Itu Seru"`

### 🟡 Challenge 4.2: Trait Bentuk
```rust
trait Bentuk {
    fn luas(&self) -> f64;
    fn nama(&self) -> String;
}
```
- Implementasikan untuk `Lingkaran`, `Persegi`, `Segitiga`
- Simpan di `Vec<Box<dyn Bentuk>>`, urutkan berdasarkan luas, lalu print
- Buat fungsi generic `fn terbesar<T: PartialOrd>(items: &[T]) -> Option<&T>`
- Tulis **minimal 5 unit test** (`#[cfg(test)]`), jalankan dengan `cargo test`

**⭐ Bonus thread:** jalankan 10 thread yang masing-masing menambah counter 1000 kali. Hasil akhir **harus tepat 10000** (pakai `Arc<Mutex<u32>>`).

### 🔴 Boss 4: Todo CLI v2 (persisten)
Upgrade Boss 3:
- Data disimpan ke `todos.json` (crate `serde` + `serde_json`), otomatis dimuat saat start
- Perintah baru: `daftar selesai`, `daftar belum`, `cari <kata>`. Semuanya dengan iterator + closure
- File JSON rusak atau tidak ada **tidak boleh bikin crash**
- Minimal **8 unit test** untuk logika `TodoList`

---

## Fase 5 — Async & Ekosistem (minggu 5)

**Materi:** `async`/`await`, Tokio, `serde`, `anyhow`/`thiserror`, `reqwest`.
📖 [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### 🟢 Challenge 5.1: Concurrent Sleep
Jalankan 3 task async yang masing-masing `sleep` 1, 2, dan 3 detik. Ukur total waktunya dengan `std::time::Instant`.

**Lulus jika:** total ≈ **3 detik**, bukan 6. Jelaskan kenapa.

### 🟡 Challenge 5.2: Fetch API
- Ambil `https://jsonplaceholder.typicode.com/todos/1` dengan `reqwest`, deserialize ke struct, lalu print
- Ambil todo id 1–10 **secara bersamaan** (`tokio::task::JoinSet` atau `futures::future::join_all`)
- Bandingkan waktunya dengan versi sequential

### 🔴 Boss 5: Pengecek Status Website
- Baca daftar URL dari `urls.txt`
- Cek semuanya **secara concurrent** dengan timeout 5 detik per URL
- Output: URL, status code, waktu respons (ms), urut dari yang tercepat
- Satu URL error **tidak boleh menghentikan** yang lain
- Error handling pakai `anyhow`

```
✅ 200  134ms  https://www.rust-lang.org
✅ 200  412ms  https://github.com
❌ timeout     https://contoh-tidak-ada.xyz
```

---

## Fase 6 — REST API dengan Axum (minggu 6–7)

**Materi:** Axum (routing, extractor `Path`/`Query`/`Json`/`State`), status code, SQLx, migration, `IntoResponse`, `tracing`, `dotenvy`.
⚠️ Axum 0.8 memakai sintaks path `/{id}`, bukan `/:id`.

### 🟢 Challenge 6.1: Endpoint Pertama
| Request | Response |
|---|---|
| `GET /` | `Halo dari Rust!` |
| `GET /halo/Budi` | `{"pesan": "Halo, Budi!"}` |
| `GET /tambah?a=2&b=3` | `{"hasil": 5}` |
| `GET /tambah?a=x` | status **400** |

### 🟡 Challenge 6.2: CRUD Todo In-Memory
State: `Arc<RwLock<Vec<Todo>>>`

| Method | Endpoint | Sukses | Gagal |
|---|---|---|---|
| GET | `/todos` | 200 | – |
| GET | `/todos/{id}` | 200 | 404 |
| POST | `/todos` | **201** | 422 jika judul kosong |
| PUT | `/todos/{id}` | 200 | 404 / 422 |
| DELETE | `/todos/{id}` | **204** | 404 |

Uji semua endpoint dengan `curl` atau Postman/Bruno. Error dikembalikan dalam format JSON: `{"error": "Todo tidak ditemukan"}`.

### 🔴 Boss 6: Todo API + Database
Upgrade 6.2:
- Simpan ke **SQLite** (atau PostgreSQL) dengan **SQLx** + migration
- Buat `enum AppError` yang implement `IntoResponse` dan pakai `?` di semua handler
- Konfigurasi (`DATABASE_URL`, `PORT`) dibaca dari `.env`
- Pagination & filter: `GET /todos?page=1&limit=10&selesai=true`
- Logging tiap request dengan `tracing` + `tower-http` `TraceLayer`
- Minimal **3 integration test** (pakai `tower::ServiceExt::oneshot`)
- Struktur folder: `routes/`, `handlers/`, `models/`, `db/`, `error.rs`

---

## Fase 7 — Final Boss (minggu 8+)

### 🔴🔴 Final Boss: Todo API Multi-User
- `POST /auth/register` dan `POST /auth/login`, password di-hash dengan `argon2`
- Login menghasilkan **JWT** (`jsonwebtoken`). Endpoint `/todos` wajib pakai header `Authorization: Bearer <token>`
- Todo milik user A **tidak bisa** dilihat atau diubah user B (harus 404, bukan 403. Pikirkan kenapa)
- `Dockerfile` multi-stage dengan image akhir < 100 MB
- `README.md` berisi dokumentasi semua endpoint + contoh `curl`

🎉 Kalau ini lulus, kamu sudah bisa membangun backend Rust yang layak dipakai sungguhan.

---

## 📋 Progress Tracker

- [ ] Fase 0 — Halo, Rustacean
- [ ] Fase 1 — 🟢 FizzBuzz · 🟡 Konverter Suhu · 🔴 Tebak Angka
- [ ] Fase 2 — 🟢 Perbaiki Error · 🟡 Fungsi Teks · 🔴 Statistik Teks
- [ ] Fase 3 — 🟢 Persegi Panjang · 🟡 Parser Perintah · 🔴 Todo CLI
- [ ] Fase 4 — 🟢 Iterator Drill · 🟡 Trait Bentuk · 🔴 Todo CLI v2
- [ ] Fase 5 — 🟢 Concurrent Sleep · 🟡 Fetch API · 🔴 Pengecek Website
- [ ] Fase 6 — 🟢 Endpoint Pertama · 🟡 CRUD In-Memory · 🔴 Todo API + DB
- [ ] Fase 7 — 🔴🔴 Todo API Multi-User

## Sumber

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings): latihan kecil, cocok dikerjakan paralel dengan Fase 1–4
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Dokumentasi Axum](https://docs.rs/axum)
