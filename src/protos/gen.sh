for i in $(find . -iname "*.proto"); do                                                                                          [19:39:52]
    new=$(echo $(dirname ${i}|sed 's#/#_#g'|sed 's/\._//g')_$(basename ${i}))
    oldimport=$(echo $(dirname ${i}|sed 's#\./##g')/$(basename ${i}))
    echo $new
    echo $oldimport
    cp $i $new
done
for i in $(find . -iname "*.proto"); do                                                                                          [19:40:41]
    new=$(echo $(dirname ${i}|sed 's#/#_#g'|sed 's/\._//g')_$(basename ${i}))
    oldimport=$(echo $(dirname ${i}|sed 's#\./##g')/$(basename ${i}))
    echo $new
    echo $oldimport
    sed -i "s#${oldimport}#${new}#g" *.proto
done
for i in $(ls *.proto); do
    protoc --rust_out . -I. ${i}
done
for i in $(grep -r message . |awk '{print $2}'); do
echo "    .type_attribute(\"ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v1alpha1.${i}\", \"#[derive(serde::Serialize, serde::Deserialize)]\")"
done

for i in $(find . -name "*.proto"); do
package=$(grep -o -P '(?<=^package ).*(?=;)' $i)
for message in $(grep -o -P '(?<=message ).*(?= {)' ${i}); do
echo "    .type_attribute(\"${package}.${message}\", \"#[derive(serde::Serialize, serde::Deserialize)]\")"
done
done