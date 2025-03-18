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

neden Rust? çünkü C'ye karşı olan hislerimiz:
[debüblüman&aleynatilki - sana güvenmiyorum](https://www.youtube.com/watch?v=XGGXlj6grzQ)

# LICENSE
[MI]()
