#!/bin/bash
DATE=`date +%Y-%m-%d`

for file in "/home/psycho/RESOURCE/归档/pdf/Safari"/* 
do
      if [ -d "$file" ]
      then
	  		 echo $file
              #7z a -r $file-$DATE.zip $file
    #   else
    #          if [ ${file: -4} == ".txt" ]       #  this is the snag
    #           #then
    #                  # do something txt-ish
    #           fi
      fi
done;

# pax> export replace=987654321
# pax> echo X123456789X | sed "s/123456789/${replace}/"
# X987654321X
# pax> _