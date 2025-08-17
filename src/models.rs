mod medical_record;
mod requirements;

pub use medical_record::{
    BaseRecord, ExtendedRecord, InsuredInfo, LabRecord, LifestyleConditionRecord, MedicalRecord,
    RecordType, RxRecord, TestRecord, ValidationRequest,
};

pub use requirements::RequirementRow;
