from enum import Enum


class PPAAgreement:
    plant_id: int
    electrolyzer_id: int


class PolicyType(Enum):
    POLICY_TYPE_PPA_AGREEMENT = 1


class Policy:
    policy_type: PolicyType
    ppa_agreement: PPAAgreement
