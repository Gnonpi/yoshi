import unittest
from check_version import SemanticVersion

# normally, i'd do that with pytest, 
# but it's a rust project, let's limit Python things
# to launch:
# python -m unittest discover

class ParseSemverTest(unittest.TestCase):
    def test_simple(self):
        semver = SemanticVersion.from_string('1.2.3')
        self.assertEqual(semver.major, 1)
        self.assertEqual(semver.minor, 2)
        self.assertEqual(semver.patch, 3)

    def test_only_patch(self):
        semver = SemanticVersion.from_string('0.0.1')
        self.assertEqual(semver.major, 0)
        self.assertEqual(semver.minor, 0)
        self.assertEqual(semver.patch, 1)

class CompareSemverTest(unittest.TestCase):
    def test_equality(self):
        semver_one = SemanticVersion(1, 2, 3)
        semver_two = SemanticVersion(1, 2, 3)
        semver_three = SemanticVersion(1, 2, 4)
        self.assertEqual(semver_one, semver_two)
        self.assertNotEqual(semver_one, semver_three)

    def test_ordering(self):
        # expected order is 1-2-3
        semver_one = SemanticVersion(1, 2, 3)
        semver_two = SemanticVersion(1, 2, 4)
        semver_three = SemanticVersion(1, 3, 2)
        list_sorted = sorted([semver_three, semver_one, semver_two])
        self.assertEqual(list_sorted, [semver_one, semver_two, semver_three])

