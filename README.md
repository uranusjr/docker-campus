# Manage all docker-compose projects on your machine in one command

## Usage

### Listing

```
$ docker-campus show --list
Name        Root
-----------------------
dearbnb     C:\Users\uranusjr\Documents\programming\dearbnb
tej-jet     C:\Users\uranusjr\Documents\programming\tej-jet

$ docker-campus show tej-jet
      Name                     Command              State            Ports
-----------------------------------------------------------------------------------
tej-jet_db_1        docker-entrypoint.sh postgres   Up      0.0.0.0:32769->5432/tcp
tej-jet_service_1   python -m tejjet.serve          Up      0.0.0.0:8000->8000/tcp
```

`docker-campus add <name> <root>` adds a new project.

`docker-campus remove <name>` removes a project.


### Management

```
$ docker-campus compose <name> <command> [<arg> ...]
```

Run `docker-compose <command> <arg> ...` on the project specified by `<name>`.
The implementation is to simply `chdir` to the project’s root directory and
call `docker-compose`.
