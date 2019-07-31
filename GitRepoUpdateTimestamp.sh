#!/bin/bash -e
####
# Helper script to update the Last modified timestamp of files in a Git SCM
# Projects working Copy
#
# When you clone a Git repository, it sets the timestamp of all the files to the
# time when you cloned the repository.
#
# This becomes a problem when you want the cloned repository, which is part of a 
# Web application have a proper cacheing mechanism so that it can re-cache files
# (into a webtree) that have been modified since the last cache.
#
# @see http://stackoverflow.com/questions/1964470/whats-the-equivalent-of-use-commit-times-for-git
#
# Author: Jeffery Fernandez <jeffery@fernandez.net.au>
####

# Make sure we are not running this on a bare Repository
REPO_TYPE=`git config --list|egrep ^core.bare | awk -F '=' '{ print $2 }'`
if [ "$REPO_TYPE" == "true" ]
then
	echo "Cannot run this script on a bare Repository" && exit 1
fi

echo "Updating Git Repository Last Modified Time-stamp"

# Obtain the Operating System
OS=${OS:-`uname`}

# Get the last revision hash of a particular file in the git repository
getFileLastRevision() 
{
	git rev-list HEAD "$1" | head -n 1
}

# Extract the actual last modified timestamp of the file and Update the time-stamp
updateFileTimeStamp() 
{
	# Extract the file revision
	FILE_REVISION_HASH=`getFileLastRevision "$1"`

	# Get the File last modified time
	FILE_MODIFIED_TIME=`git show --pretty=format:%ai --abbrev-commit ${FILE_REVISION_HASH} | head -n 1`
	
	# Extract the last modified timestamp, differently for Linux, FreeBSD and Mac OS X
	if [ "$OS" = 'Linux' ]
	then
		# for displaying the date in readable format
		#FORMATTED_TIMESTAMP=`date --date="${FILE_MODIFIED_TIME}" +'%d-%m-%Y %H:%M:%S %z'`
		#echo "Modified: ${FILE_MODIFIED_TIME} | ${FORMATTED_TIMESTAMP} > ${1}"
		
		# Modify the last modified timestamp
		touch -d "${FILE_MODIFIED_TIME}" $2
	
	elif [ "$OS" = 'Darwin' ] || [ "$OS" = 'FreeBSD' ]
	then
		# Format the date for updating the timestamp
		FORMATTED_TIMESTAMP=`date -j -f '%Y-%m-%d %H:%M:%S %z' "${FILE_MODIFIED_TIME}" +'%Y%m%d%H%M.%S'`
		#echo "Modified: ${FILE_MODIFIED_TIME} | ${FORMATTED_TIMESTAMP} > ${1}"
		
		# Modify the last modified timestamp
		touch -t  "${FORMATTED_TIMESTAMP}" $2
	else
		echo "Unknown Operating System to perform timestamp update" >&2
		exit 1
	fi
}

# Backup and update the "Internal Field Separator" to a newline. This is so that
# we can deal with spaces in file names in the for loop below
IFS_BAK=$IFS
IFS="
"
# Loop through and fix timestamps on all files in our checked-out repository
for file in $(git ls-files)
do
	updateFileTimeStamp "${file}" "${file}"
done

# Revert the default delimiter
IFS=$IFS_BAK
IFS_BAK=
