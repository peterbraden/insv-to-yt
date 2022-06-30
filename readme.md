# insv-to-yt

Convert Insta360 `.insv` videos to mp4 that can be uploaded to youtube as 360
video.

Insta360 videos are `.insv` files which are mp4 in fisheye format. You need 2
files to reconstruct the full 360 degree range.

Youtube requires a `.mp4` with custom metadata in an equirectangular projection.

We therefore need to:

1. Strip the trailer data from the `.insv` file and rename to `.mp4`
2. Join the 2 files side by side as dual fisheye.
3. Remap each frame to equirectangular.
4. Add the custom youtube metadata.

Out of scope for the moment:

- Reprojecting the focus of the video.

