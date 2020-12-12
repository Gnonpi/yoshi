'''
Small script to compare the git tags and the version in Cargo.toml

If there's already a tag with a version inferior or equal to the one in toml,
we stop

The goal is not to have a brillant Python demonstration,
that's why I'm using prints and unittest and keeping it simple
'''
import re
from pathlib import Path
from typing import List
from subprocess import check_output

RE_SEMVER = re.compile(r'(?P<major>\d{1,2})\.(?P<minor>\d{1,2})\.(?P<patch>\d{1,2})')
PATH_CARGO_TOML = Path(__file__).resolve().parents[1].joinpath('Cargo.toml')

# todo: should I use a library? for now, it might be lighter and faster to have a custom class
class SemanticVersion:
    '''
    Class representing any semantic version
    '''

    def __init__(self, major: int, minor: int, patch: int):
        self.major = major
        self.minor = minor
        self.patch = patch

    @classmethod
    def from_string(cls, semver_string: str) -> 'SemanticVersion':
        match = RE_SEMVER.match(semver_string)
        if match is None:
            raise ValueError(f"Could not parse semantic version from '{semver_string}'")
        matches = match.groupdict()
        return cls(
            int(matches['major']), 
            int(matches['minor']), 
            int(matches['patch'])
        )

    def __eq__(self, other):
        return self.major == other.major and self.minor == other.minor and self.patch == other.patch

    def __lt__(self, other):
        if self.major < other.major:
            return True
        if self.minor < other.minor:
            return True
        return self.patch < other.patch

    def __le__(self, other):
        return self == other or self < other
        
    def __repr__(self):
        return f"SemanticVersion(major={self.major}, minor={self.minor}, patch={self.patch})"


def read_git_tags() -> List[SemanticVersion]:
    """
    Check current local git tags and parse them into SemanticVersion
    """
    tag_list_bytes = check_output(['git', 'tag', '--list'])
    tag_list = tag_list_bytes.decode("utf-8").split('\n')
    tag_list = [tag for tag in tag_list if tag != '']
    tag_list_versions = [SemanticVersion.from_string(tag) for tag in tag_list]
    return tag_list_versions

def read_cargo_toml() -> SemanticVersion:
    """
    Read version written in current Cargo.toml
    """
    print(f'Reading Cargo.toml at {PATH_CARGO_TOML}')
    with open(PATH_CARGO_TOML, 'r') as f:
        # if the crate version is not the first version key, this is wrong
        for line in f:
            if line.startswith('version'):
                sem_ver = line.split('=')[1].strip().strip('\"')
                return SemanticVersion.from_string(sem_ver)
    raise KeyError(f'Found no semantic version in Cargo.toml')

def main():
    git_tag_versions = read_git_tags()
    print(f'Got {len(git_tag_versions)} from git tags')
    
    toml_version = read_cargo_toml()
    print(f'Got {toml_version} from Cargo.toml')
    
    if len(git_tag_versions) == 0:
        print('Since we got no git tags, version should be ok')
    
    if toml_version in git_tag_versions:
        raise RuntimeError(f"Version in toml '{toml_version}' already found in git tags")
    
    max_git_tag_version = max(git_tag_versions)
    if toml_version <= max_git_tag_version:
        raise RuntimeError(f"Version in toml '{toml_version}' lower or equal to max git tag '{max_git_tag_version}'")
    print('Everything seems fine')


if __name__ == '__main__':
    main()
