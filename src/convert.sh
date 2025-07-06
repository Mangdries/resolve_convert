#If something goes wrong, please refer to the Arch Wiki:
#https://wiki.archlinux.org/title/DaVinci_Resolve#MP4,_H.264,_H.265_and_AAC_Support
#Alternatively, the source of this program can be modified...

arg="$1"
no_ext="${arg:0:-4}"
output="${no_ext}.mov"

echo "$output"
ls -l "$arg"

ffmpeg -i "$1" -c:v dnxhd -profile:v dnxhr_hq -pix_fmt yuv422p -c:a alac "$output"
