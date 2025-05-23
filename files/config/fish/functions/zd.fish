#zd - go to project directory

function zd
	if not test $argv[1]
		cd 
		return
	end
	if test $argv[1] = "-"
		cd -
		return
	end
	set path $argv[1]
	set project_path $(project_manager print project $argv[1] | grep Path)
	set project_path (string replace -r "Path: " "" $project_path)
	if test $project_path
		echo "moving to project $argv[1] at $path"
		set path $project_path
	end

	cd $path
end

set project_names $(project_manager print projects -p)
