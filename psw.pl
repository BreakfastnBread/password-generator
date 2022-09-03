#!/usr/bin/perl -w
use strict;
use warnings;
my @adjectives = qw("adorable" "adventurous" "aggressive" "agreeable" "alert" "alive" "amused" "angry" 
"annoyed" "annoying" "anxious" "arrogant" "ashamed" "attractive" "average" "awful"
"bad" "beautiful" "better" "bewildered" "black" "bloody" "blue" "blushing" "bored"
"brainy" "brave" "breakable" "bright" "eager" "easy" "elated" "elegant" "embarrassed"
"enchanting" "encouraging" "energetic" "enthusiastic" "envious" "evil" "excited");
my @nouns = qw("time" "person" "year" "way" "day" "thing" "man" "world" "life" "hand" "part" "child" "eye"
"woman" "place" "work" "week" "case" "point" "government" "company" "number" "group" 
"problem" "fact" "Aardvark" "Aardwolf" "Abyssinian" "Manga" "Addax" "Penguin" "snake"
"Affenpinscher" "Hound" "Bullfrog" "Civet" "Frog" "Elephant" "Parrot" "beetle");
my @symbols = qw(! @ $ % ^ & * - _ ~ ?);
my @split_adj = split('', $adjectives[rand(int($#adjectives))]); #split string by char
my @split_noun = split('', $nouns[rand(int($#nouns))]);
@split_adj = @split_adj[1..($#split_adj-1)]; #remove quotes from array
@split_noun = @split_noun[1..($#split_noun-1)];
$split_adj[rand(int($#split_adj))] = $symbols[rand(int($#symbols))];
$split_noun[rand(int($#split_noun))] = $symbols[rand(int($#symbols))];
my $final_adj = join('', @split_adj);
my $final_noun = join('', @split_noun);
print("$final_adj"."_");
for(my $i=0; $i<3; $i++) {
    print(int(rand(9)));
}
print("_$final_noun\n");

