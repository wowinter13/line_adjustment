# line_adjustment

### Task

Написать алгоритм, принимающий на вход строку разделенную пробелами, и длину строки в символах.
Необходимо разбить исходный текст на строки и выравнять по указанной длине строки с помощью пробелов.
Расстояние между словами нужно заполнять равным количеством пробелов, если же это не возможно, то добавляем
еще по пробелу между словами слева направо. Если в строке помещается только 1 слово, то дополнить строку 
пробелами справа. Результат вернуть в виде единой строки, где полученный список равных по ширине строк склеен 
с помощью символа перевода строки.

Реализовать максимально производительное решение при сохранении читабельности кода, такого чтобы его можно было использовать в продакшене и поддерживать в дальнейшем.

### Running tests

`cargo test`

### Further improvements (things to consider/discuss)


#### Memory usage

- We use `Vec<&str>` to store words temporarily. This is not optimal, because we could use `&str` directly. However, we need to store words somewhere, and `Vec<&str>` is the most convenient way to do it.
- We could use `String::with_capacity()` to preallocate memory for the output string. However, this would require us to calculate the exact size of the output string. This would be a tradeoff between memory usage and performance. I decided to leave it as is, because it's not a big deal for test task.


Do what you must...I will watch you.


<p align="center">
<img width="300" height="500" src="https://static.wikia.nocookie.net/elderscrolls/images/b/ba/Imperial_Prison_Guard.png/revision/latest?cb=20131214131751">
</p>