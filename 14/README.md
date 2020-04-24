# Morse code decoder

I looked at my sound file in audacity and can see the samples. There's no noise and a perfect 550Hz sin way making the beeps. I have also looked up what a wav file looks like. The approach I need to take is to:

1. Read the header from the file, as this contains the sample rate and how big each sample is etc.
2. Read the samples 1 by 1, when there's no nothing and the sample is very low it's < 0.1 amplitude.
3. When the samples are > 0.1 it's a beep, so count how many samples > 0.1 there are to determine the length of the beep. 
    * In this file the shorts are roughly 0.6s long and the longs are 1.8s long. A space between words is roughly 1.2s long. 
4. Read in the lengths until a space is encountered. When it is I output the letter and keep reading the samples.


## Next steps
* How to get live data from the microphone? 
    * So I can decode something in real time.
* Is there a standard timing for a long/short/space that it might adhere to?
    * This will guide how to determine longs and shorts for more than my one clean file
* Is there a standard frequency of the signal?
    * This will guide any filtering I need to do.
* Can I produce a wav file containing my morse code message?
    * I could output it to the speakers too?
