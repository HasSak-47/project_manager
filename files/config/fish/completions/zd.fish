# get all projects in format "project:path"
set projects $(project_manager print projects)
set dirs */

for project in $projects
	set split $(string split "," $project)
	set path $split[2]
	complete -c zd -a $split[1]
end

for dir in $dirs
	complete -c zd -a $dir
end
