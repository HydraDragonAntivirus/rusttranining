// https://www.youtube.com/watch?v=0t_jpl70Mbg&list=PLgvWD2scL860_6ppZQS6i86vQuX_5wV2
// Main fonksiyonunu oluşturmak istediğimiz için function main dekserasyonu oluşturuyoruz ve parantezler içerisinde de arıyorum ilk başta main fonksiyonunu arıyor birde süslü parantezleri arıyoruz
//tam olarak fonksiyon denemez kırmızı çizgimiz noktalı ve her zaman koymalıyız 
//fn main() {
 //   println!("Hello, rust!");
//} // ders 1 function fonksiyon main fonskiyonun println! ile print ediyoruz ve ;'yı unutmamak lazım
//ders 2
//fn main () {
    // mut olsun ki değiştirebilirim sonradan
    //let mut age = 30; // value assigned to age is never read çünkü onu 33 yaptım
    //age = 33; //sonradan değiştirdim
    //const PI_NUMBER: f32 = 3.14159; // pi sayısı
  //  println!("{} {}" ,PI_NUMBER, age); // {]} ve age ile çağırdım veya PI Number ile iki kez çağırma
//} 
/* ders1 tekrar */
//fn main ()  {
 //    println!("Hello world!");}
//ders 3 i8 signed u8 unsigned but 8 bit 816 16 bit 32 bit 64 bit 128 bit diye gidiyor u128 vb.
// arch isize usize 255 0 arası 2n-1 signed sayılar -128 127 -(2n-1) 2n-1-1
// ihtiyacınıza uygun değeri seçin büyük olursa 16x maliyet arttırmış
//fn main () {
//  let age:u8 = 30;
//  println!("{:#b}",age);
//}
//ders 9'a kadar geldim
// doğru değişken şeyleri vb. 9 ders diğer dersleri mantık kurarak baya vakit harcayarak anladım sonra ai'ye sorara sora kodları öğreneceğim
// I finished the course yay!
fn main () {
let mut x = 5;
println!("x'in degeri: {}",x);
x = 6;
println!("x'in degeri 2{}",x);
 print!("Hello World!");
 selam_ver("Dunya");
}
fn selam_ver (isim: &str) {
println! ( "Merhaba, {}!",isim);
}
fn text () {
    let sayi = 3;
    if sayi < 5 {
        println! ("sayi 5'den kucukmus");
     } else {
     println!("sayi 5'ten buyuk veya esit")
    }
}
