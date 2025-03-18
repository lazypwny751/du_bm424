# derleyici tasarım notlar

## nesting level
tekrar eden elemanların ne kadar tekrar ettiğini anlamak için kullanırız, mesela iç içe geçen if'ler var diyelim başta nesting level = 0 her bir if geldiğinde buna +1 eklenecek, şimdi burada da scope için geçerli şöyle anlatayım:

```
level = 0
{	// level = 1
	{	level = 2	
		
	}	// level = 1
}  // level = 0
```
Günün sonundan nesting level başlangıç değeri ne ise o olmalıdır yoksa, biyerde bir hata var demektir.

## derleyici vs yorumlayıcı vs JIT derleyici

### derleyiciler
derleyiciler adamdır, makine kodu üretir hızlıdır lakin derlenme süreleri zaman alabilir eskiden:
lex -> parse -> ast -> code gen -> assembler -> pure machine code
artık daha kolay.
lex -> parse -> ast gen -> ir code gen -> code gen -> assembler -> pure machine code
intermediate representation sayesinde yorumlayıcılardaki mimari avantajı kanadı demek yanlış olmaz.

### yorumlayıcılar
yorumlayıcılar da adamdır lakin yorumlayıcılar şöyle çalışıyor,
lex/parse -> ast -> code execution.

şimdi burada code execution kısmı biraz üstü kapalı gelmiş olabilir, eskiden direk olarak ast üstüne runtime'la bu gerçekleştiriliyormuştu, ast'den "3" + "5" geldiyse arkaplanda bi runtime fonksiyonu bunları topluyor, bu sıkıntı. Sonradan bu VM bazlı oldu ve her mimariye ayrı kod yazmak yerine vm'i her mimaride çalışacak şekilde yaptılar ve ast'den gelen kodu vm'de çalıştırdılar yani "3" + "5"'i doğrudan makine bazlı çalıştırmak yerine vm'den çalıştırmasını istediler vm kodu optimize ederek uygun mimaride çalıştırdı çıktıyı sağladı.

neden Rust? çünkü C'ye karşı olan hislerimiz:
[debüblüman&aleynatilki - sana güvenmiyorum](https://www.youtube.com/watch?v=XGGXlj6grzQ)

# LICENSE
[MIT]()
