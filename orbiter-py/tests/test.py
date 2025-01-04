import unittest
import orbiter
# python -m unittest discover tests


class TestOrbiter(unittest.TestCase):
    def test_orbiter(self):
        with orbiter.Orbiter() as orb:
            orb.test()


if __name__ == '__main__':
    unittest.main()
