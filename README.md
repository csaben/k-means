# k-means in rust


## features

- randomly initialize o(n * k)
- ++ initialize o(n^2 * k)
- ++ initialize o(n * k) curtesy of claude

## usage

To see speed comparison try:
`cargo test speed_test -- --nocapture`

## resources
- [Wikipedia](https://en.wikipedia.org/wiki/K-means_clustering)
- [Centroid Initialization Methods for k-means Clustering](https://www.kdnuggets.com/2020/06/centroid-initialization-k-means-clustering.html)
- [Stack Overflow](https://stackoverflow.com/a/5468119)
