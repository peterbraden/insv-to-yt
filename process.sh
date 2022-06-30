
v1=test/VID_20211112_112308_00_153.insv
v2=test/VID_20211112_112308_10_153.insv

# Join side by side
#ffmpeg -i $v1 -i $v2 -filter_complex "[0:v][1:v]hstack=inputs=2[v]; [0:a][1:a]amerge[a]" -map "[v]" -map "[a]" -ac 2 intermediate.mp4



# Remap to equirectangular
ffmpeg -i intermediate.mp4 -vf v360=dfisheye:e:yaw=-90 intermediate2.mp4
