import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("fuzz_smoke", ROOT / "scripts/fuzz_smoke.py")
FUZZ = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(FUZZ)


class FuzzSmokeTests(unittest.TestCase):
    def setUp(self):
        self.fixtures = json.loads((ROOT / "fixtures/core/records-v1.json").read_text())

    def test_record_seeds_have_every_selector_and_both_shapes(self):
        seeds = FUZZ.seeds("record_codec", self.fixtures)
        self.assertEqual(len(seeds), 78)
        for index in range(13):
            for shape, data in zip(("full", "minimal"), seeds[2 * index:2 * index + 2]):
                self.assertEqual(data[0], index)
                self.assertEqual(json.loads(data[1:]), self.fixtures[index][shape])

    def test_record_import_seeds_exercise_read_only_and_rejected_paths_without_mutating_fixtures(self):
        original = json.dumps(self.fixtures, sort_keys=True)
        seeds = FUZZ.seeds("record_codec", self.fixtures)
        for index in range(13):
            for shape, data in zip(("full", "minimal"), seeds[26 + 2 * index:28 + 2 * index]):
                self.assertEqual(data[0], index)
                expected = dict(self.fixtures[index][shape])
                expected.update(schema_version={"major": 1, "minor": 1},
                                future_only={"unrecognized_permission": False})
                self.assertEqual(json.loads(data[1:]), expected)
            self.assertEqual(seeds[52 + index][0], index)
            self.assertEqual(json.loads(seeds[52 + index][1:])["schema_version"], {"major": 2, "minor": 0})
            self.assertEqual(seeds[65 + index][0], index)
            self.assertIn(b'"x":0,"x":1', seeds[65 + index])
        self.assertEqual(json.dumps(self.fixtures, sort_keys=True), original)

    def test_duplicate_and_missing_seed_families_are_rejected(self):
        for fixtures in (self.fixtures[:-1], self.fixtures + self.fixtures[:1]):
            with self.assertRaises(ValueError):
                FUZZ.seeds("record_codec", fixtures)

    def test_envelope_seeds_contain_every_record(self):
        values = FUZZ.seeds("control_envelope", self.fixtures)
        for fixture, data in zip(self.fixtures, values):
            self.assertEqual(json.loads(data)["payload"], fixture["full"])
        self.assertIn(b'{"a":1,"\\u0061":2}', values)

    def test_frame_lengths_are_big_endian_and_match_payloads(self):
        values = FUZZ.seeds("frame_decoder", self.fixtures)
        for data in values[:13]:
            self.assertEqual(data[0], 1)
            self.assertEqual(int.from_bytes(data[1:5], "big"), len(data) - 5)
            self.assertEqual(json.loads(data[5:])["message"], {"kind": "event"})

    def test_success_requires_executed_instrumented_inputs(self):
        log = "INFO: Loaded 1 modules (900 inline 8-bit counters)\n#101 DONE cov: 300 ft: 600\nstat::number_of_executed_units: 101\n"
        self.assertEqual(FUZZ.result_summary(log, 0)["result"], "success")
        for text, code in [(log, 1), ("Finished release profile", 0), ("cov: 9", 0),
                           (log.replace("stat::number_of_executed_units: 101", ""), 0),
                           (log.replace("INFO: Loaded 1 modules (900 inline 8-bit counters)", ""), 0)]:
            self.assertEqual(FUZZ.result_summary(text, code)["result"], "failure")


if __name__ == "__main__":
    unittest.main()
