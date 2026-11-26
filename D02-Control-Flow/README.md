# 🦀 Day 02: Control Flow (If Else & Looping)

> **Target hari ini:** bisa mengatur alur program dengan percabangan (`if`, `else`, `match`) dan perulangan (`loop`, `while`, `for`).

## Daftar Isi
1. [Apa itu Control Flow?](#1-apa-itu-control-flow)
2. [Operator Perbandingan & Logika](#2-operator-perbandingan--logika)
3. [If Else](#3-if-else)
4. [Match (alternatif if else)](#4-match-alternatif-if-else)
5. [Looping: Gambaran Umum](#5-looping-gambaran-umum)
6. [loop](#6-loop)
7. [while](#7-while)
8. [for](#8-for)
9. [break, continue & Label](#9-break-continue--label)
10. [Error yang Sering Muncul](#10-error-yang-sering-muncul)
11. [Rangkuman](#11-rangkuman)
12. [Challenge](#12-challenge)

---

## 1. Apa itu Control Flow?

### Definisi
**Control flow** (alur kontrol) adalah cara mengatur **urutan eksekusi kode**. Secara default kode dijalankan dari atas ke bawah. Dengan control flow, program bisa:

| Jenis | Fungsi | Kata kunci |
|---|---|---|
| **Percabangan** | Memilih kode mana yang dijalankan berdasarkan kondisi | `if`, `else`, `match` |
| **Perulangan** | Menjalankan kode yang sama berkali-kali | `loop`, `while`, `for` |
| **Lompatan** | Menghentikan atau melewati perulangan | `break`, `continue`, `return` |

---

## 2. Operator Perbandingan & Logika

Kondisi dalam control flow selalu menghasilkan **`bool`** (`true` / `false`).

### Perbandingan
| Operator | Arti | Contoh | Hasil |
|---|---|---|---|
| `==` | Sama dengan | `5 == 5` | `true` |
| `!=` | Tidak sama dengan | `5 != 3` | `true` |
| `>` | Lebih besar | `5 > 3` | `true` |
| `<` | Lebih kecil | `5 < 3` | `false` |
| `>=` | Lebih besar atau sama | `5 >= 5` | `true` |
| `<=` | Lebih kecil atau sama | `3 <= 5` | `true` |

### Logika
| Operator | Arti | Contoh | Hasil |
|---|---|---|---|
| `&&` | AND: semua harus `true` | `true && false` | `false` |
| `\|\|` | OR: minimal satu `true` | `true \|\| false` | `true` |
| `!` | NOT: membalik nilai | `!true` | `false` |

> 💡 `&&` dan `||` bersifat **short-circuit**: di `a && b`, kalau `a` sudah `false`, maka `b` tidak dicek lagi.

> ⚠️ Kedua sisi perbandingan **harus bertipe sama**. `5_i32 == 5.0` error, karena `i32` tidak bisa dibandingkan dengan `f64`.

---

## 3. If Else

### Definisi
`if` menjalankan sebuah blok kode **hanya jika** kondisinya `true`.

### 3.1 `if` saja
```rust
let nilai = 85;

if nilai >= 70 {
    println!("Lulus");
}
```

### 3.2 `if` + `else`
```rust
if nilai >= 70 {
    println!("Lulus");
} else {
    println!("Tidak lulus");
}
```

### 3.3 `if` + `else if` + `else`
Kondisi dicek **berurutan dari atas**. Begitu ada yang `true`, sisanya dilewati.
```rust
if nilai >= 90 {
    println!("A");
} else if nilai >= 80 {
    println!("B");      // ← 85 masuk di sini, cabang di bawahnya tidak dicek lagi
} else if nilai >= 70 {
    println!("C");
} else {
    println!("D");
}
```

> ⚠️ **Urutan penting.** Kalau `nilai >= 70` ditulis paling atas, nilai 95 akan masuk ke cabang "C".

### 3.4 Menggabungkan kondisi
```rust
let umur = 20;
let punya_sim = true;

if umur >= 17 && punya_sim {
    println!("Boleh menyetir");
}

if umur < 13 || umur > 60 {
    println!("Dapat diskon");
} else {
    println!("Harga normal");
}

if !punya_sim {
    println!("Buat SIM dulu");
}
```

### 3.5 If bersarang (nested)
```rust
let angka = 15;

if angka > 0 {
    if angka % 2 == 0 {
        println!("Positif genap");
    } else {
        println!("Positif ganjil");
    }
} else {
    println!("Nol atau negatif");
}
```
> 💡 Kalau sudah bersarang lebih dari 2 tingkat, kode jadi sulit dibaca. Pertimbangkan `&&`, `match`, atau *early return*.

### 3.6 ⭐ `if` sebagai expression
Di Rust, `if` **menghasilkan nilai**, sehingga bisa langsung disimpan ke variabel. Ini pengganti operator ternary `kondisi ? a : b` yang tidak ada di Rust.

```rust
let status = if nilai >= 75 { "lulus" } else { "remedial" };
let jenis = if angka % 2 == 0 { "genap" } else { "ganjil" };
let terbesar = if a > b { a } else { b };

println!("{}", if angka % 2 == 0 { "Even" } else { "Odd" });
```

**Aturan if sebagai expression:**
1. **Wajib ada `else`**, karena nilainya harus selalu ada
2. **Semua cabang harus bertipe sama**
3. Nilai terakhir di setiap blok **tidak** diberi `;`

```rust
let s = if x > 1 { "a" };              // ❌ E0317: `if` may be missing an `else` clause
let s = if x > 1 { 1 } else { "a" };   // ❌ E0308: `if` and `else` have incompatible types
```

### 3.7 `if` sebagai return function
```rust
fn grade(n: i32) -> char {
    if n >= 90 {
        'A'
    } else if n >= 80 {
        'B'
    } else if n >= 70 {
        'C'
    } else {
        'D'
    }
}
```

### 3.8 Early return
Keluar dari function lebih awal untuk menghindari `if` bersarang:
```rust
fn cek_umur(umur: i32) {
    if umur < 0 {
        println!("Umur tidak valid");
        return;               // langsung keluar dari function
    }
    println!("Umur: {umur}");
}
```

### 3.9 Sekilas: `if let`
Cara singkat untuk mengecek **satu pola** saja. Biasanya dipakai dengan `Option` atau `Result` (Fase 3).
```rust
let input = "42";

if let Ok(n) = input.parse::<i32>() {
    println!("Angka: {n}");
} else {
    println!("Bukan angka");
}
```

### Perbedaan `if` di Rust dengan bahasa lain
| | Rust | JavaScript / PHP / C |
|---|---|---|
| Kurung di kondisi | Tidak perlu: `if x > 5 {` | Wajib: `if (x > 5) {` |
| Kurung kurawal `{}` | **Wajib**, walaupun isinya 1 baris | Opsional untuk 1 baris |
| Kondisi | **Harus `bool`** | Angka atau string juga bisa (`if (1)`, `if ("a")`) |
| Ternary `? :` | Tidak ada, pakai `if` expression | Ada |

---

## 4. Match (alternatif if else)

`match` membandingkan sebuah nilai dengan beberapa **pola**. Cocok sebagai pengganti `if else if` yang panjang.

```rust
let angka = 3;

match angka {
    1 => println!("satu"),
    2 | 3 => println!("dua atau tiga"),     // | = atau
    4..=9 => println!("empat sampai sembilan"),
    _ => println!("lainnya"),               // _ = semua sisanya (wajib kalau belum semua tercakup)
}
```

### Match juga expression
```rust
let suhu = 30;
let kondisi = match suhu {
    ..=0 => "beku",
    1..=25 => "sejuk",
    _ => "panas",
};
```

### Match dengan kondisi tambahan (guard)
```rust
let teks = match angka {
    n if n < 0 => "negatif",
    0 => "nol",
    1..=9 => "satuan",
    _ => "puluhan ke atas",
};
```

### Kapan pakai `if` atau `match`?
| Situasi | Pakai |
|---|---|
| 1–2 kondisi sederhana | `if` / `else` |
| Kondisi berupa rentang atau gabungan (`&&`, `\|\|`) dari beberapa variabel | `if` / `else if` |
| Membandingkan **satu nilai** dengan banyak kemungkinan | `match` |
| Bekerja dengan `enum`, `Option`, `Result` | `match` |

> 💡 `match` wajib **exhaustive** (mencakup semua kemungkinan). Compiler akan error kalau ada kasus yang terlewat, sehingga bug karena lupa satu kondisi bisa dicegah.

---

## 5. Looping: Gambaran Umum

Rust punya **3 jenis perulangan**:

| | `loop` | `while` | `for` |
|---|---|---|---|
| Berhenti saat | Ada `break` | Kondisi `false` | Elemen habis |
| Jumlah putaran | Belum pasti | Belum pasti | Sudah diketahui |
| Bisa mengembalikan nilai (`break nilai`) | ✅ | ❌ | ❌ |
| Risiko loop tanpa henti | Ada | Ada | Tidak ada |
| Paling cocok untuk | Menu, input ulang, retry | Selama kondisi benar | Range, array, string, Vec |

---

## 6. `loop`

### Definisi
`loop` mengulang **terus-menerus tanpa henti** sampai dihentikan dengan `break`.

```rust
loop {
    println!("Selamanya!");   // Ctrl+C untuk menghentikan
}
```

### 6.1 Berhenti dengan `break`
```rust
let mut hitung = 0;

loop {
    hitung += 1;
    println!("Putaran {hitung}");
    if hitung == 3 {
        break;
    }
}
```

### 6.2 ⭐ `loop` bisa mengembalikan nilai
```rust
let mut hitung = 0;

let hasil = loop {
    hitung += 1;
    if hitung == 5 {
        break hitung * 10;   // nilai setelah break menjadi hasil loop
    }
};

println!("{hasil}");         // 50
```

### 6.3 Contoh nyata: minta input sampai benar
Pola ini sudah dipakai di `control.rs`:
```rust
fn number_input(prompt: &str) -> i32 {
    loop {
        let text = input(prompt);
        match text.parse::<i32>() {
            Ok(n) => return n,                                  // keluar dari loop dan function
            Err(_) => println!("{text} is not a number. try again"),
        }
    }
}
```

### 6.4 Contoh: menu program
```rust
loop {
    let pilihan = input("1. Mulai  2. Keluar → ");
    match pilihan.as_str() {
        "1" => println!("Mulai!"),
        "2" => break,
        _ => println!("Pilihan tidak ada"),
    }
}
```

---

## 7. `while`

### Definisi
`while` mengulang **selama kondisinya `true`**. Kondisi dicek **sebelum** setiap putaran.

```rust
let mut n = 3;

while n > 0 {
    print!("{n}.. ");
    n -= 1;
}
println!("🚀");   // 3.. 2.. 1.. 🚀
```

> ⚠️ Pastikan kondisinya bisa menjadi `false`. Kalau lupa `n -= 1`, loop **tidak pernah berhenti**.

### 7.1 Contoh: jumlah putaran tidak diketahui
```rust
let mut saldo = 100;
let mut bulan = 0;

while saldo > 0 {
    saldo -= 30;
    bulan += 1;
}

println!("Saldo habis dalam {bulan} bulan");   // 4 bulan
```

### 7.2 Kalau kondisi sudah `false` sejak awal
```rust
let mut x = 10;
while x < 5 {
    println!("Tidak pernah jalan");
}
```

### 7.3 Sekilas: `while let`
Mengulang selama pola masih cocok:
```rust
let mut tumpukan = vec![1, 2, 3];

while let Some(x) = tumpukan.pop() {   // berhenti saat pop() menghasilkan None
    print!("{x} ");                     // 3 2 1
}
```

### `while` vs `loop`
```rust
// Sama persis:
while kondisi { /* ... */ }

loop {
    if !kondisi { break; }
    /* ... */
}
```
Pakai `while` kalau kondisinya bisa ditulis di awal, supaya lebih mudah dibaca.

---

## 8. `for`

### Definisi
`for` mengulang **setiap elemen** dari sesuatu yang bisa diulang (*iterable*): range, array, Vec, atau string.

```rust
for variabel in sesuatu_yang_bisa_diulang {
    // kode
}
```

> 💡 Rust **tidak punya** `for (i = 0; i < n; i++)` seperti C atau JavaScript. `for` di Rust selalu berbentuk `for ... in ...`, dan ini adalah perulangan yang paling sering dan paling aman dipakai.

### 8.1 Range (angka)
| Kode | Hasil | Keterangan |
|---|---|---|
| `for i in 1..4` | `1 2 3` | `..` → angka akhir **tidak** ikut |
| `for i in 1..=4` | `1 2 3 4` | `..=` → angka akhir **ikut** |
| `for i in (1..=4).rev()` | `4 3 2 1` | Mundur |
| `for i in (0..=10).step_by(3)` | `0 3 6 9` | Lompat per 3 |
| `for i in (0..=10).rev().step_by(5)` | `10 5 0` | Mundur sambil lompat |
| `for c in 'a'..='e'` | `a b c d e` | Range huruf |
| `for i in 0..n` | `0` sampai `n-1` | Batasnya boleh berupa variabel |
| `for i in 5..1` | *(tidak jalan)* | Awal > akhir → kosong, **bukan** mundur |

```rust
for i in 1..=5 {
    println!("Putaran ke-{i}");
}

for _ in 0..3 {         // `_` kalau variabelnya tidak dipakai
    println!("Hai");
}
```

### 8.2 Array / Vec
```rust
let buah = ["apel", "jeruk", "mangga"];

// Nilainya saja
for b in buah {
    println!("{b}");
}

// Index + nilai
for (i, b) in buah.iter().enumerate() {
    println!("{}. {}", i + 1, b);      // 1. apel, 2. jeruk, 3. mangga
}
```

> ⚠️ Hindari gaya C seperti ini:
> ```rust
> for i in 0..buah.len() { println!("{}", buah[i]); }   // jalan, tapi tidak idiomatik
> ```
> Pakai `for b in buah` atau `enumerate()`.

### 8.3 Mengubah isi saat diulang
```rust
let mut angka = [1, 2, 3];

for x in &mut angka {     // atau: angka.iter_mut()
    *x *= 10;             // * = ubah nilai yang ditunjuk
}

println!("{angka:?}");    // [10, 20, 30]
```

### 8.4 ⚠️ Jebakan: Vec "habis dipakai" setelah di-loop
```rust
let v = vec![String::from("a"), String::from("b")];

for s in v { }            // v DIPINDAHKAN ke dalam loop
println!("{}", v.len());  // ❌ error: borrow of moved value: `v`
```
Solusinya, **pinjam** dengan `&`:
```rust
for s in &v { }           // ✅ hanya meminjam
println!("{}", v.len());  // ✅ 2
```

| Cara | Tipe `x` | Bisa ubah isi? | Koleksi masih bisa dipakai? |
|---|---|---|---|
| `for x in v` | Nilai asli | – | ❌ (untuk Vec/String) |
| `for x in &v` / `v.iter()` | `&T` (pinjam) | ❌ | ✅ |
| `for x in &mut v` / `v.iter_mut()` | `&mut T` | ✅ | ✅ |

Penjelasan lengkapnya di Fase 2 (Ownership). Untuk sekarang: **kalau masih mau dipakai setelah loop, pakai `&`**.

### 8.5 String
```rust
for c in "Halo".chars() { }                        // per karakter: H a l o
for kata in "saya suka rust".split_whitespace() { } // per kata
for baris in "satu\ndua".lines() { }               // per baris
```
> ⚠️ `for c in "Halo"` tanpa `.chars()` akan error. Harus dipilih mau diulang per apa.

### 8.6 Loop bersarang
```rust
for i in 1..=3 {
    for j in 1..=3 {
        print!("{} ", i * j);
    }
    println!();
}
```
```
1 2 3
2 4 6
3 6 9
```

### 8.7 Sekilas: for + method iterator
```rust
// Dua array sekaligus
let nama = ["Ana", "Budi"];
let nilai = [90, 80];
for (n, s) in nama.iter().zip(nilai.iter()) {
    println!("{n}: {s}");
}

// Hanya yang memenuhi syarat
for x in (1..=10).filter(|x| x % 3 == 0) { }   // 3 6 9

// Ubah dulu sebelum diulang
for x in (1..=3).map(|x| x * x) { }            // 1 4 9

// Loop penjumlahan bisa diganti satu baris
let total: i32 = (1..=100).sum();              // 5050
```
`|x| ...` adalah **closure**, dipelajari di Fase 4.

---

## 9. `break`, `continue` & Label

| Kata kunci | Fungsi |
|---|---|
| `break` | Hentikan loop sepenuhnya |
| `break nilai` | Hentikan `loop` sambil mengembalikan nilai (**hanya untuk `loop`**) |
| `continue` | Lewati sisa kode, langsung ke putaran berikutnya |
| `return` | Keluar dari **function** (otomatis ikut keluar dari semua loop) |

```rust
for i in 1..=10 {
    if i % 2 == 0 { continue; }   // lewati angka genap
    if i > 7 { break; }            // berhenti
    print!("{i} ");                // 1 3 5 7
}
```

### Contoh: cari lalu berhenti
```rust
let arr = [3, 9, 2];
let mut posisi = None;

for (i, &x) in arr.iter().enumerate() {
    if x == 9 {
        posisi = Some(i);
        break;
    }
}
println!("{posisi:?}");   // Some(1)
```

### Label: keluar dari loop paling luar
Tanpa label, `break` hanya menghentikan loop yang **paling dalam**.
```rust
'luar: for i in 1..=3 {
    for j in 1..=3 {
        if i * j == 4 {
            println!("Ketemu {i} x {j}");
            break 'luar;          // hentikan loop 'luar
        }
    }
}
```
- Label diawali tanda kutip satu: `'luar`, `'cari`, `'baris`
- Bisa dipakai di `loop`, `while`, dan `for`
- `continue 'luar;` juga bisa

---

## 10. Error yang Sering Muncul

| Error | Penyebab | Solusi |
|---|---|---|
| `E0308: mismatched types, expected bool` | `if x { }` dengan `x` berupa angka | Tulis kondisinya: `if x != 0 { }` |
| `E0317: if may be missing an else clause` | `if` dipakai sebagai nilai tapi tidak ada `else` | Tambahkan `else` |
| `E0308: if and else have incompatible types` | Cabang `if` dan `else` bertipe berbeda | Samakan tipenya |
| `E0571: break with value from a for loop` | `break nilai` di dalam `for` atau `while` | Pakai `loop`, atau simpan ke variabel `mut` |
| `E0004: non-exhaustive patterns` | `match` belum mencakup semua kemungkinan | Tambahkan `_ => ...` |
| `borrow of moved value` | Vec/String dipakai setelah `for x in v` | Pakai `for x in &v` |
| Program tidak berhenti | Kondisi `while` selalu `true` atau `loop` tanpa `break` | Pastikan variabel kondisinya berubah. Tekan Ctrl+C untuk menghentikan |

---

## 11. Rangkuman

- [ ] Kondisi `if` **harus `bool`** dan tidak perlu kurung `()`
- [ ] Kurung kurawal `{}` **wajib**, walaupun isinya 1 baris
- [ ] `else if` dicek berurutan dari atas, jadi urutannya penting
- [ ] `if` adalah expression: `let x = if a { 1 } else { 2 };`
- [ ] If expression wajib punya `else`, dan semua cabangnya bertipe sama
- [ ] `match` untuk membandingkan satu nilai dengan banyak pola, wajib exhaustive
- [ ] `loop` mengulang tanpa henti sampai `break`, dan bisa `break nilai`
- [ ] `while` mengulang selama kondisinya `true`
- [ ] `for` mengulang setiap elemen dan paling aman
- [ ] `1..5` tidak termasuk 5, `1..=5` termasuk 5
- [ ] Pakai `for x in &v` kalau `v` masih mau dipakai setelah loop
- [ ] `continue` melewati putaran, `break` menghentikan loop
- [ ] Label `'nama:` untuk keluar dari loop bersarang

---

## 12. Challenge

### 🟢 1. Kalkulator Nilai
Input nilai 0–100, lalu tampilkan grade:
| Nilai | Grade |
|---|---|
| 90–100 | A |
| 80–89 | B |
| 70–79 | C |
| 60–69 | D |
| < 60 | E |

Nilai di luar 0–100 → tampilkan `Nilai tidak valid`. Buat **dua versi**: dengan `if else` dan dengan `match`.

### 🟢 2. Tabel Perkalian
Input sebuah angka, lalu cetak tabel perkalian 1 sampai 10 dengan `for`:
```
Angka: 7
7 x 1 = 7
7 x 2 = 14
...
7 x 10 = 70
```

### 🟢 3. Hitung Mundur
Input angka, lalu hitung mundur dengan **`while`** sampai 0, kemudian cetak `🚀 Meluncur!`

### 🟡 4. Segitiga Bintang
```
Tinggi: 4
*
**
***
****
```
⭐ Bonus: versi terbalik dan piramida rata tengah.

💡 Hint: `"*".repeat(i)`. Untuk piramida: `" ".repeat(tinggi - i)` + `"*".repeat(2 * i - 1)`.

### 🟡 5. Statistik Nilai
Dari `let nilai = [78, 92, 65, 88, 70, 95, 60];`, **pakai `for`** (jangan `.sum()` atau `.max()`) untuk menghitung:
- Total dan rata-rata (`f64`, 2 angka di belakang koma)
- Nilai tertinggi dan terendah
- Jumlah yang lulus (≥ 70)

### 🟡 6. Hitung Huruf
Input sebuah kalimat, lalu hitung jumlah huruf vokal, konsonan, angka, dan spasi.

### 🔴 Boss: Menu Kalkulator
Program dengan menu yang **terus berulang** (pakai `loop`) sampai user memilih keluar:
```
=== KALKULATOR ===
1. Tambah
2. Kurang
3. Kali
4. Bagi
5. Keluar
Pilih: 4
Angka pertama: 10
Angka kedua: 0
❌ Tidak bisa membagi dengan nol
```
Syarat:
- Pilihan menu memakai `match`
- Input yang bukan angka **tidak boleh** membuat crash (pakai `number_input`)
- Pembagian dengan nol ditangani
- Saat keluar, tampilkan berapa kali kalkulator dipakai

### ✍️ Mini Quiz
1. Apa hasil `if 5 > 3 && 2 > 4 { "a" } else { "b" }`?
2. Kenapa `let x = if true { 1 };` error?
3. Berapa kali `println!` dijalankan pada `for i in 3..3 { println!("{i}"); }`?
4. Apa beda `break` di `loop` dengan `break` di `for`?
5. Apa isi variabel `hasil`?
   ```rust
   let mut n = 1;
   let hasil = loop {
       n *= 2;
       if n > 20 { break n; }
   };
   ```

**Jawaban:**
> 1. _..._
> 2. _..._
> 3. _..._
> 4. _..._
> 5. _..._
