using CSV, DataFrames, JSON3;

# Read CSV to DataFrame
df = DataFrame(CSV.File("word/WordSmart2.csv"; delim=":", header=false));

# Set Header
rename!(df, [:word, :mean])

# Shuffle
dg = df[shuffle(axes(df, 1)), :]

# JSON3
n_chapter = div(length(axes(dg, 1)), 40) + 1;
total = Vector{Dict}(undef, n_chapter);
for i in 1:n_chapter-1
    chapter = 21 + i;
    start = 1 + (i - 1) * 40;
    final = i * 40;
    dh = dg[start:final, :];
    word = dh[!, :word];
    mean = dh[!, :mean];
    words = Vector{Dict}(undef, 40);
    for j in 1:40
        words[j] = Dict("word" => word[j], "mean" => mean[j]);
    end
    dict = Dict(
        "chapter" => chapter,
        "words" => words,
    )
    total[i] = dict;
end

last_chapter = 21 + n_chapter;
last_start = 1 + (n_chapter - 1) * 40;
last_final = length(axes(dg, 1));
dh = dg[last_start:last_final, :];
word = dh[!, :word];
mean = dh[!, :mean];
last_words = Vector{Dict}(undef, length(word));
for j in 1:length(word)
    last_words[j] = Dict("word" => word[j], "mean" => mean[j]);
end
last_dict = Dict(
    "chapter" => last_chapter,
    "words" => last_words,
)
total[end] = last_dict;

wordsmart = Dict("total" => total);

# write a pretty file
open("word/word2.json", "w") do f
    JSON3.pretty(f, JSON3.write(wordsmart))
    println(f)
end