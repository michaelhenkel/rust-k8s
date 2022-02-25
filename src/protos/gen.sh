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
for i in $(ls *.proto); do                                                                                                       [19:41:12]
    protoc --rust_out . -I. ${i}
done
