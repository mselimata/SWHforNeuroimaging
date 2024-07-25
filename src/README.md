# Java querrying

[doc](https://docs.softwareheritage.org/devel/swh-graph/java-api.html)

## [SWHID](https://docs.softwareheritage.org/devel/swh-model/persistent-identifiers.html)
Objects come in different types:

- contents

- directories

- revisions

- releases

- snapshots


## Nix env with vscode and java

get the jdk path, and add it to `.vscode/settings.json`, under `java.jdt.ls.java.home`.

to get the jdk path : `echo $JAVA_HOME`, for example `/nix/store/qjcx43a8fcgybb2bnxc4vcprb42zf6hx-openjdk-17.0.7+7/lib/openjdk`
> the extension language server want absoluly use its own jdk if the setting isnt set
> env variable arent working in `settings.json`

Maven seams to not understand we arent at the root of the project, so the dependencies problem (incompatible) arent real

## Results

The run on the [python graphs](https://docs.softwareheritage.org/devel/swh-dataset/graph/dataset.html#graph-dataset-2019-01-28-popular-3k-python) (28-01-2019) yields the following :
```shell
their is 45691499 nodes.
node id: 7228909, node SWHID : swh:1:snp:bf7b8caee7859fea276cc68c7de91c872cd1e158
node id: 7260193, node SWHID : swh:1:snp:6547673c1f4864a95b7cf64d64f841b049b7b8d3
node id: 7260195, node SWHID : swh:1:snp:2537ddbf7f20e632c0fbf2dc77694e167f3f4f8e
node id: 7331978, node SWHID : swh:1:snp:1b227fb90374b90abae8a7d407ae4d9355e97ca6
node id: 7350723, node SWHID : swh:1:snp:69d98ffb4504dc7de573fff3cdd49d51bb337d4f
node id: 7350724, node SWHID : swh:1:snp:a12ead7d4296e85301df9170052ef72463891de3
node id: 7350726, node SWHID : swh:1:snp:11fe7e4a9314ccd89c463a5f60a638e2a0e9dcd2
node id: 7350729, node SWHID : swh:1:snp:0da5088420a77a3a3ace6ed5d6732ef56157bb73
node id: 7350730, node SWHID : swh:1:snp:7d00da22ce365e0ad87f92b92b79fbd1e275a1a8
node id: 7350732, node SWHID : swh:1:snp:591355c4188a6e265bbb171be5e3cdee5b11309c
their is 22187 nodes of type SNP
it took 40.703857 seconds to iterate over the nodes.
```
[
Use the global [endpoint](https://archive.softwareheritage.org/api/1/graph/) :
> WARN : Need [authentification](https://archive.softwareheritage.org/oidc/profile/#tokens)
```shell
(source /home/clahoche/Documents/data/swh/.env && curl -i -H "Authorization: Bearer ${TOCKEN}" https://archive.softwareheritage.org/api/1/graph/graph/stats/; unset TOKEN)

```
]

> Need to use the [bucket s3](https://docs.softwareheritage.org/devel/swh-dataset/graph/dataset.html#graph-dataset-2023-09-06) and get the file stats
> Nodes in the total graph : 34121566250
> execution of the java for all the graph : 30396.88746600382s, 506.61479110006366h, 21.10894962916932 jours