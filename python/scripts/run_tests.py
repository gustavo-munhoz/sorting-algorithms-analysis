import unittest
import sys

if __name__ == "__main__":
    loader = unittest.TestLoader()
    suite = loader.loadTestsFromName("tests.test_sort_algorithms")
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)

    # Se qualquer falha ou erro, saímos com código 1
    if not result.wasSuccessful():
        sys.exit(1)