use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CreateExternalTable {
    pub table_name: OwnedTableReference,
    pub if_not_exists: bool,
    pub table_options: TableOptions,
    pub tunnel: Option<String>,
}

impl TryFrom<protogen::sqlexec::logical_plan::CreateExternalTable> for CreateExternalTable {
    type Error = ProtoConvError;

    fn try_from(
        proto: protogen::sqlexec::logical_plan::CreateExternalTable,
    ) -> std::result::Result<Self, Self::Error> {
        let table_name = proto
            .table_name
            .ok_or(ProtoConvError::RequiredField(
                "table_name is required".to_string(),
            ))?
            .try_into()?;

        let tbl_options = proto.table_options.ok_or(ProtoConvError::RequiredField(
            "table_options is required".to_string(),
        ))?;

        Ok(Self {
            table_name,
            if_not_exists: proto.if_not_exists,
            table_options: tbl_options.try_into()?,
            tunnel: proto.tunnel,
        })
    }
}

impl UserDefinedLogicalNodeCore for CreateExternalTable {
    fn name(&self) -> &str {
        Self::EXTENSION_NAME
    }

    fn inputs(&self) -> Vec<&DfLogicalPlan> {
        vec![]
    }

    fn schema(&self) -> &datafusion::common::DFSchemaRef {
        &EMPTY_SCHEMA
    }

    fn expressions(&self) -> Vec<datafusion::prelude::Expr> {
        vec![]
    }

    fn fmt_for_explain(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::EXTENSION_NAME)
    }

    fn from_template(
        &self,
        _exprs: &[datafusion::prelude::Expr],
        _inputs: &[DfLogicalPlan],
    ) -> Self {
        self.clone()
    }
}

impl ExtensionNode for CreateExternalTable {
    const EXTENSION_NAME: &'static str = "CreateExternalTable";

    fn try_decode_extension(extension: &LogicalPlanExtension) -> Result<Self> {
        match extension.node.as_any().downcast_ref::<Self>() {
            Some(s) => Ok(s.clone()),
            None => Err(internal!(
                "CreateExternalTable::try_from_extension: unsupported extension",
            )),
        }
    }

    fn try_encode(&self, buf: &mut Vec<u8>, _codec: &dyn LogicalExtensionCodec) -> Result<()> {
        use protogen::sqlexec::logical_plan as protogen;

        let create_table = protogen::CreateExternalTable {
            table_name: self.table_name.clone().try_into().ok(),
            if_not_exists: self.if_not_exists,
            table_options: Some(self.table_options.clone().try_into().ok().ok_or(
                ProtoConvError::RequiredField("table_options is required".to_string()),
            )?),
            tunnel: self.tunnel.clone(),
        };

        let extension = protogen::LogicalPlanExtensionType::CreateExternalTable(create_table);

        let lp_extension = protogen::LogicalPlanExtension {
            inner: Some(extension),
        };

        lp_extension
            .encode(buf)
            .map_err(|e| internal!("{}", e.to_string()))?;

        Ok(())
    }
}