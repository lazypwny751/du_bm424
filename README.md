# derleyici tasarım notlar

## nesting level
tekrar eden elemanların ne kadar tekrar ettiğini anlamak için kullanırız, mesela iç içe geçen if'ler var diyelim başta nesting level = 0 her bir if geldiğinde buna +1 eklenecek, şimdi burada da scope için geçerli şöyle basit bir örnekle izah etmek gerekirse:

```
level = 0
{	// level = 1
	{	level = 2	
		
	}	// level = 1
}  // level = 0
```
Günün sonundan nesting level başlangıç değeri ne ise o olmalıdır yoksa, biyerlerde bir hata var demektir.

## derleyici vs yorumlayıcı

### derleyiciler
derleyiciler adamdır, makine kodu üretir hızlıdır lakin derlenme süreleri zaman alabilir eskiden:
lex -> parse -> ast -> code gen -> assembler -> pure machine code
artık daha kolay.
lex -> parse -> ast gen -> ir code gen -> code gen -> assembler -> pure machine code
intermediate representation sayesinde yorumlayıcılardaki mimari avantajı kanadı demek yanlış olmaz.

### yorumlayıcılar
yorumlayıcılar da adamdır lakin yorumlayıcılar şöyle çalışıyor,
lex/parse -> ast -> code execution.

code execution kısmı biraz üstü kapalı gelmiş olabilir, eskiden direkt olarak AST üstüne runtime ile bu gerçekleştiriliyormuş, AST bağlamında eğer "3" + "5" geldiyse arkaplanda runtime fonksiyonu bunları topluyor, bu sıkıntı. Sonradan bu VM bazlı oldu ve her mimariye ayrı kod yazmak yerine vm'i her mimaride çalışacak şekilde yaptılar ve ast'den gelen kodu vm'de çalıştırdılar yani "3" + "5"'i doğrudan makine bazlı çalıştırmak yerine vm'den çalıştırmasını istediler vm kodu optimize ederek uygun mimaride çalıştırdı çıktıyı sağladı.

![kita rust](https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books/blob/master/Rust/Ikuyo_Kita_holding_Rust_Book.png?raw=true)

# LICENSE
[MIT]()
