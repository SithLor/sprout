pub const CRC16_TABLE: [u16; 256] = [
    0x0, 0xC0C1, 0xC181, 0x140, 
    0xC301, 0x3C0, 0x280, 0xC241, 
    0xC601, 0x6C0, 0x780, 0xC741, 
    0x500, 0xC5C1, 0xC481, 0x440, 
    0xCC01, 0xCC0, 0xD80, 0xCD41, 
    0xF00, 0xCFC1, 0xCE81, 0xE40, 
    0xA00, 0xCAC1, 0xCB81, 0xB40, 
    0xC901, 0x9C0, 0x880, 0xC841, 
    0xD801, 0x18C0, 0x1980, 0xD941, 
    0x1B00, 0xDBC1, 0xDA81, 0x1A40, 
    0x1E00, 0xDEC1, 0xDF81, 0x1F40, 
    0xDD01, 0x1DC0, 0x1C80, 0xDC41, 
    0x1400, 0xD4C1, 0xD581, 0x1540, 
    0xD701, 0x17C0, 0x1680, 0xD641, 
    0xD201, 0x12C0, 0x1380, 0xD341, 
    0x1100, 0xD1C1, 0xD081, 0x1040, 
    0xF001, 0x30C0, 0x3180, 0xF141, 
    0x3300, 0xF3C1, 0xF281, 0x3240, 
    0x3600, 0xF6C1, 0xF781, 0x3740, 
    0xF501, 0x35C0, 0x3480, 0xF441, 
    0x3C00, 0xFCC1, 0xFD81, 0x3D40, 
    0xFF01, 0x3FC0, 0x3E80, 0xFE41, 
    0xFA01, 0x3AC0, 0x3B80, 0xFB41, 
    0x3900, 0xF9C1, 0xF881, 0x3840, 
    0x2800, 0xE8C1, 0xE981, 0x2940, 
    0xEB01, 0x2BC0, 0x2A80, 0xEA41, 
    0xEE01, 0x2EC0, 0x2F80, 0xEF41, 
    0x2D00, 0xEDC1, 0xEC81, 0x2C40, 
    0xE401, 0x24C0, 0x2580, 0xE541, 
    0x2700, 0xE7C1, 0xE681, 0x2640, 
    0x2200, 0xE2C1, 0xE381, 0x2340, 
    0xE101, 0x21C0, 0x2080, 0xE041, 
    0xA001, 0x60C0, 0x6180, 0xA141, 
    0x6300, 0xA3C1, 0xA281, 0x6240, 
    0x6600, 0xA6C1, 0xA781, 0x6740, 
    0xA501, 0x65C0, 0x6480, 0xA441, 
    0x6C00, 0xACC1, 0xAD81, 0x6D40, 
    0xAF01, 0x6FC0, 0x6E80, 0xAE41, 
    0xAA01, 0x6AC0, 0x6B80, 0xAB41, 
    0x6900, 0xA9C1, 0xA881, 0x6840, 
    0x7800, 0xB8C1, 0xB981, 0x7940, 
    0xBB01, 0x7BC0, 0x7A80, 0xBA41, 
    0xBE01, 0x7EC0, 0x7F80, 0xBF41, 
    0x7D00, 0xBDC1, 0xBC81, 0x7C40, 
    0xB401, 0x74C0, 0x7580, 0xB541, 
    0x7700, 0xB7C1, 0xB681, 0x7640, 
    0x7200, 0xB2C1, 0xB381, 0x7340, 
    0xB101, 0x71C0, 0x7080, 0xB041, 
    0x5000, 0x90C1, 0x9181, 0x5140, 
    0x9301, 0x53C0, 0x5280, 0x9241, 
    0x9601, 0x56C0, 0x5780, 0x9741, 
    0x5500, 0x95C1, 0x9481, 0x5440, 
    0x9C01, 0x5CC0, 0x5D80, 0x9D41, 
    0x5F00, 0x9FC1, 0x9E81, 0x5E40, 
    0x5A00, 0x9AC1, 0x9B81, 0x5B40, 
    0x9901, 0x59C0, 0x5880, 0x9841, 
    0x8801, 0x48C0, 0x4980, 0x8941, 
    0x4B00, 0x8BC1, 0x8A81, 0x4A40, 
    0x4E00, 0x8EC1, 0x8F81, 0x4F40, 
    0x8D01, 0x4DC0, 0x4C80, 0x8C41, 
    0x4400, 0x84C1, 0x8581, 0x4540, 
    0x8701, 0x47C0, 0x4680, 0x8641, 
    0x8201, 0x42C0, 0x4380, 0x8341, 
    0x4100, 0x81C1, 0x8081, 0x4040, 
];
pub const CRC32_TABLE: [u32; 256] = [
    0x0, 0x77073096, 0xEE0E612C, 0x990951BA, 
    0x76DC419, 0x706AF48F, 0xE963A535, 0x9E6495A3, 
    0xEDB8832, 0x79DCB8A4, 0xE0D5E91E, 0x97D2D988, 
    0x9B64C2B, 0x7EB17CBD, 0xE7B82D07, 0x90BF1D91, 
    0x1DB71064, 0x6AB020F2, 0xF3B97148, 0x84BE41DE, 
    0x1ADAD47D, 0x6DDDE4EB, 0xF4D4B551, 0x83D385C7, 
    0x136C9856, 0x646BA8C0, 0xFD62F97A, 0x8A65C9EC, 
    0x14015C4F, 0x63066CD9, 0xFA0F3D63, 0x8D080DF5, 
    0x3B6E20C8, 0x4C69105E, 0xD56041E4, 0xA2677172, 
    0x3C03E4D1, 0x4B04D447, 0xD20D85FD, 0xA50AB56B, 
    0x35B5A8FA, 0x42B2986C, 0xDBBBC9D6, 0xACBCF940, 
    0x32D86CE3, 0x45DF5C75, 0xDCD60DCF, 0xABD13D59, 
    0x26D930AC, 0x51DE003A, 0xC8D75180, 0xBFD06116, 
    0x21B4F4B5, 0x56B3C423, 0xCFBA9599, 0xB8BDA50F, 
    0x2802B89E, 0x5F058808, 0xC60CD9B2, 0xB10BE924, 
    0x2F6F7C87, 0x58684C11, 0xC1611DAB, 0xB6662D3D, 
    0x76DC4190, 0x1DB7106, 0x98D220BC, 0xEFD5102A, 
    0x71B18589, 0x6B6B51F, 0x9FBFE4A5, 0xE8B8D433, 
    0x7807C9A2, 0xF00F934, 0x9609A88E, 0xE10E9818, 
    0x7F6A0DBB, 0x86D3D2D, 0x91646C97, 0xE6635C01, 
    0x6B6B51F4, 0x1C6C6162, 0x856530D8, 0xF262004E, 
    0x6C0695ED, 0x1B01A57B, 0x8208F4C1, 0xF50FC457, 
    0x65B0D9C6, 0x12B7E950, 0x8BBEB8EA, 0xFCB9887C, 
    0x62DD1DDF, 0x15DA2D49, 0x8CD37CF3, 0xFBD44C65, 
    0x4DB26158, 0x3AB551CE, 0xA3BC0074, 0xD4BB30E2, 
    0x4ADFA541, 0x3DD895D7, 0xA4D1C46D, 0xD3D6F4FB, 
    0x4369E96A, 0x346ED9FC, 0xAD678846, 0xDA60B8D0, 
    0x44042D73, 0x33031DE5, 0xAA0A4C5F, 0xDD0D7CC9, 
    0x5005713C, 0x270241AA, 0xBE0B1010, 0xC90C2086, 
    0x5768B525, 0x206F85B3, 0xB966D409, 0xCE61E49F, 
    0x5EDEF90E, 0x29D9C998, 0xB0D09822, 0xC7D7A8B4, 
    0x59B33D17, 0x2EB40D81, 0xB7BD5C3B, 0xC0BA6CAD, 
    0xEDB88320, 0x9ABFB3B6, 0x3B6E20C, 0x74B1D29A, 
    0xEAD54739, 0x9DD277AF, 0x4DB2615, 0x73DC1683, 
    0xE3630B12, 0x94643B84, 0xD6D6A3E, 0x7A6A5AA8, 
    0xE40ECF0B, 0x9309FF9D, 0xA00AE27, 0x7D079EB1, 
    0xF00F9344, 0x8708A3D2, 0x1E01F268, 0x6906C2FE, 
    0xF762575D, 0x806567CB, 0x196C3671, 0x6E6B06E7, 
    0xFED41B76, 0x89D32BE0, 0x10DA7A5A, 0x67DD4ACC, 
    0xF9B9DF6F, 0x8EBEEFF9, 0x17B7BE43, 0x60B08ED5, 
    0xD6D6A3E8, 0xA1D1937E, 0x38D8C2C4, 0x4FDFF252, 
    0xD1BB67F1, 0xA6BC5767, 0x3FB506DD, 0x48B2364B, 
    0xD80D2BDA, 0xAF0A1B4C, 0x36034AF6, 0x41047A60, 
    0xDF60EFC3, 0xA867DF55, 0x316E8EEF, 0x4669BE79, 
    0xCB61B38C, 0xBC66831A, 0x256FD2A0, 0x5268E236, 
    0xCC0C7795, 0xBB0B4703, 0x220216B9, 0x5505262F, 
    0xC5BA3BBE, 0xB2BD0B28, 0x2BB45A92, 0x5CB36A04, 
    0xC2D7FFA7, 0xB5D0CF31, 0x2CD99E8B, 0x5BDEAE1D, 
    0x9B64C2B0, 0xEC63F226, 0x756AA39C, 0x26D930A, 
    0x9C0906A9, 0xEB0E363F, 0x72076785, 0x5005713, 
    0x95BF4A82, 0xE2B87A14, 0x7BB12BAE, 0xCB61B38, 
    0x92D28E9B, 0xE5D5BE0D, 0x7CDCEFB7, 0xBDBDF21, 
    0x86D3D2D4, 0xF1D4E242, 0x68DDB3F8, 0x1FDA836E, 
    0x81BE16CD, 0xF6B9265B, 0x6FB077E1, 0x18B74777, 
    0x88085AE6, 0xFF0F6A70, 0x66063BCA, 0x11010B5C, 
    0x8F659EFF, 0xF862AE69, 0x616BFFD3, 0x166CCF45, 
    0xA00AE278, 0xD70DD2EE, 0x4E048354, 0x3903B3C2, 
    0xA7672661, 0xD06016F7, 0x4969474D, 0x3E6E77DB, 
    0xAED16A4A, 0xD9D65ADC, 0x40DF0B66, 0x37D83BF0, 
    0xA9BCAE53, 0xDEBB9EC5, 0x47B2CF7F, 0x30B5FFE9, 
    0xBDBDF21C, 0xCABAC28A, 0x53B39330, 0x24B4A3A6, 
    0xBAD03605, 0xCDD70693, 0x54DE5729, 0x23D967BF, 
    0xB3667A2E, 0xC4614AB8, 0x5D681B02, 0x2A6F2B94, 
    0xB40BBE37, 0xC30C8EA1, 0x5A05DF1B, 0x2D02EF8D, 
];
pub const CRC64_TABLE: [u64; 256] = [
    0x0, 0x3C3B78E888D80FE1, 0x7876F1D111B01FC2, 0x444D893999681023, 
    0x750C207570B452A3, 0x4937589DF86C5D42, 0xD7AD1A461044D61, 0x3141A94CE9DC4280, 
    0x6FF9833DB2BCC861, 0x53C2FBD53A64C780, 0x178F72ECA30CD7A3, 0x2BB40A042BD4D842, 
    0x1AF5A348C2089AC2, 0x26CEDBA04AD09523, 0x62835299D3B88500, 0x5EB82A715B608AE1, 
    0x5A12C5AC36ADFDE5, 0x6629BD44BE75F204, 0x2264347D271DE227, 0x1E5F4C95AFC5EDC6, 
    0x2F1EE5D94619AF46, 0x13259D31CEC1A0A7, 0x5768140857A9B084, 0x6B536CE0DF71BF65, 
    0x35EB469184113584, 0x9D03E790CC93A65, 0x4D9DB74095A12A46, 0x71A6CFA81D7925A7, 
    0x40E766E4F4A56727, 0x7CDC1E0C7C7D68C6, 0x38919735E51578E5, 0x4AAEFDD6DCD7704, 
    0x31C4488F3E8F96ED, 0xDFF3067B657990C, 0x49B2B95E2F3F892F, 0x7589C1B6A7E786CE, 
    0x44C868FA4E3BC44E, 0x78F31012C6E3CBAF, 0x3CBE992B5F8BDB8C, 0x85E1C3D753D46D, 
    0x5E3DCBB28C335E8C, 0x6206B35A04EB516D, 0x264B3A639D83414E, 0x1A70428B155B4EAF, 
    0x2B31EBC7FC870C2F, 0x170A932F745F03CE, 0x53471A16ED3713ED, 0x6F7C62FE65EF1C0C, 
    0x6BD68D2308226B08, 0x57EDF5CB80FA64E9, 0x13A07CF2199274CA, 0x2F9B041A914A7B2B, 
    0x1EDAAD56789639AB, 0x22E1D5BEF04E364A, 0x66AC5C8769262669, 0x5A97246FE1FE2988, 
    0x42F0E1EBA9EA369, 0x381476F63246AC88, 0x7C59FFCFAB2EBCAB, 0x4062872723F6B34A, 
    0x71232E6BCA2AF1CA, 0x4D18568342F2FE2B, 0x955DFBADB9AEE08, 0x356EA7525342E1E9, 
    0x6388911E7D1F2DDA, 0x5FB3E9F6F5C7223B, 0x1BFE60CF6CAF3218, 0x27C51827E4773DF9, 
    0x1684B16B0DAB7F79, 0x2ABFC98385737098, 0x6EF240BA1C1B60BB, 0x52C9385294C36F5A, 
    0xC711223CFA3E5BB, 0x304A6ACB477BEA5A, 0x7407E3F2DE13FA79, 0x483C9B1A56CBF598, 
    0x797D3256BF17B718, 0x45464ABE37CFB8F9, 0x10BC387AEA7A8DA, 0x3D30BB6F267FA73B, 
    0x399A54B24BB2D03F, 0x5A12C5AC36ADFDE, 0x41ECA5635A02CFFD, 0x7DD7DD8BD2DAC01C, 
    0x4C9674C73B06829C, 0x70AD0C2FB3DE8D7D, 0x34E085162AB69D5E, 0x8DBFDFEA26E92BF, 
    0x5663D78FF90E185E, 0x6A58AF6771D617BF, 0x2E15265EE8BE079C, 0x122E5EB66066087D, 
    0x236FF7FA89BA4AFD, 0x1F548F120162451C, 0x5B19062B980A553F, 0x67227EC310D25ADE, 
    0x524CD9914390BB37, 0x6E77A179CB48B4D6, 0x2A3A28405220A4F5, 0x160150A8DAF8AB14, 
    0x2740F9E43324E994, 0x1B7B810CBBFCE675, 0x5F3608352294F656, 0x630D70DDAA4CF9B7, 
    0x3DB55AACF12C7356, 0x18E224479F47CB7, 0x45C3AB7DE09C6C94, 0x79F8D39568446375, 
    0x48B97AD9819821F5, 0x7482023109402E14, 0x30CF8B0890283E37, 0xCF4F3E018F031D6, 
    0x85E1C3D753D46D2, 0x346564D5FDE54933, 0x7028EDEC648D5910, 0x4C139504EC5556F1, 
    0x7D523C4805891471, 0x416944A08D511B90, 0x524CD9914390BB3, 0x391FB5719CE10452, 
    0x67A79F00C7818EB3, 0x5B9CE7E84F598152, 0x1FD16ED1D6319171, 0x23EA16395EE99E90, 
    0x12ABBF75B735DC10, 0x2E90C79D3FEDD3F1, 0x6ADD4EA4A685C3D2, 0x56E6364C2E5DCC33, 
    0x42F0E1EBA9EA3693, 0x7ECB990321323972, 0x3A86103AB85A2951, 0x6BD68D2308226B0, 
    0x37FCC19ED95E6430, 0xBC7B97651866BD1, 0x4F8A304FC8EE7BF2, 0x73B148A740367413, 
    0x2D0962D61B56FEF2, 0x11321A3E938EF113, 0x557F93070AE6E130, 0x6944EBEF823EEED1, 
    0x580542A36BE2AC51, 0x643E3A4BE33AA3B0, 0x2073B3727A52B393, 0x1C48CB9AF28ABC72, 
    0x18E224479F47CB76, 0x24D95CAF179FC497, 0x6094D5968EF7D4B4, 0x5CAFAD7E062FDB55, 
    0x6DEE0432EFF399D5, 0x51D57CDA672B9634, 0x1598F5E3FE438617, 0x29A38D0B769B89F6, 
    0x771BA77A2DFB0317, 0x4B20DF92A5230CF6, 0xF6D56AB3C4B1CD5, 0x33562E43B4931334, 
    0x217870F5D4F51B4, 0x3E2CFFE7D5975E55, 0x7A6176DE4CFF4E76, 0x465A0E36C4274197, 
    0x7334A9649765A07E, 0x4F0FD18C1FBDAF9F, 0xB4258B586D5BFBC, 0x3779205D0E0DB05D, 
    0x6388911E7D1F2DD, 0x3A03F1F96F09FD3C, 0x7E4E78C0F661ED1F, 0x427500287EB9E2FE, 
    0x1CCD2A5925D9681F, 0x20F652B1AD0167FE, 0x64BBDB88346977DD, 0x5880A360BCB1783C, 
    0x69C10A2C556D3ABC, 0x55FA72C4DDB5355D, 0x11B7FBFD44DD257E, 0x2D8C8315CC052A9F, 
    0x29266CC8A1C85D9B, 0x151D14202910527A, 0x51509D19B0784259, 0x6D6BE5F138A04DB8, 
    0x5C2A4CBDD17C0F38, 0x6011345559A400D9, 0x245CBD6CC0CC10FA, 0x1867C58448141F1B, 
    0x46DFEFF5137495FA, 0x7AE4971D9BAC9A1B, 0x3EA91E2402C48A38, 0x29266CC8A1C85D9, 
    0x33D3CF8063C0C759, 0xFE8B768EB18C8B8, 0x4BA53E517270D89B, 0x779E46B9FAA8D77A, 
    0x217870F5D4F51B49, 0x1D43081D5C2D14A8, 0x590E8124C545048B, 0x6535F9CC4D9D0B6A, 
    0x54745080A44149EA, 0x684F28682C99460B, 0x2C02A151B5F15628, 0x1039D9B93D2959C9, 
    0x4E81F3C86649D328, 0x72BA8B20EE91DCC9, 0x36F7021977F9CCEA, 0xACC7AF1FF21C30B, 
    0x3B8DD3BD16FD818B, 0x7B6AB559E258E6A, 0x43FB226C074D9E49, 0x7FC05A848F9591A8, 
    0x7B6AB559E258E6AC, 0x4751CDB16A80E94D, 0x31C4488F3E8F96E, 0x3F273C607B30F68F, 
    0xE66952C92ECB40F, 0x325DEDC41A34BBEE, 0x761064FD835CABCD, 0x4A2B1C150B84A42C, 
    0x1493366450E42ECD, 0x28A84E8CD83C212C, 0x6CE5C7B54154310F, 0x50DEBF5DC98C3EEE, 
    0x619F161120507C6E, 0x5DA46EF9A888738F, 0x19E9E7C031E063AC, 0x25D29F28B9386C4D, 
    0x10BC387AEA7A8DA4, 0x2C87409262A28245, 0x68CAC9ABFBCA9266, 0x54F1B14373129D87, 
    0x65B0180F9ACEDF07, 0x598B60E71216D0E6, 0x1DC6E9DE8B7EC0C5, 0x21FD913603A6CF24, 
    0x7F45BB4758C645C5, 0x437EC3AFD01E4A24, 0x7334A9649765A07, 0x3B08327EC1AE55E6, 
    0xA499B3228721766, 0x3672E3DAA0AA1887, 0x723F6AE339C208A4, 0x4E04120BB11A0745, 
    0x4AAEFDD6DCD77041, 0x7695853E540F7FA0, 0x32D80C07CD676F83, 0xEE374EF45BF6062, 
    0x3FA2DDA3AC6322E2, 0x399A54B24BB2D03, 0x47D42C72BDD33D20, 0x7BEF549A350B32C1, 
    0x25577EEB6E6BB820, 0x196C0603E6B3B7C1, 0x5D218F3A7FDBA7E2, 0x611AF7D2F703A803, 
    0x505B5E9E1EDFEA83, 0x6C6026769607E562, 0x282DAF4F0F6FF541, 0x1416D7A787B7FAA0, 
];
pub const CRC128_TABLE: [u128; 256] = [
    0x0, 0x3C3B78E888D80FE1463B78E888D80FE1, 0x7876F1D111B01FC28C76F1D111B01FC2, 0x444D893999681023CA4D893999681023, 
    0x750C207570B452A39D0C207570B452A3, 0x4937589DF86C5D42DB37589DF86C5D42, 0xD7AD1A461044D61117AD1A461044D61, 0x3141A94CE9DC42805741A94CE9DC4280, 
    0x6FF9833DB2BCC861BFF9833DB2BCC861, 0x53C2FBD53A64C780F9C2FBD53A64C780, 0x178F72ECA30CD7A3338F72ECA30CD7A3, 0x2BB40A042BD4D84275B40A042BD4D842, 
    0x1AF5A348C2089AC222F5A348C2089AC2, 0x26CEDBA04AD0952364CEDBA04AD09523, 0x62835299D3B88500AE835299D3B88500, 0x5EB82A715B608AE1E8B82A715B608AE1, 
    0x5A12C5AC36ADFDE5FA12C5AC36ADFDE5, 0x6629BD44BE75F204BC29BD44BE75F204, 0x2264347D271DE2277664347D271DE227, 0x1E5F4C95AFC5EDC6305F4C95AFC5EDC6, 
    0x2F1EE5D94619AF46671EE5D94619AF46, 0x13259D31CEC1A0A721259D31CEC1A0A7, 0x5768140857A9B084EB68140857A9B084, 0x6B536CE0DF71BF65AD536CE0DF71BF65, 
    0x35EB46918411358445EB469184113584, 0x9D03E790CC93A6503D03E790CC93A65, 0x4D9DB74095A12A46C99DB74095A12A46, 0x71A6CFA81D7925A78FA6CFA81D7925A7, 
    0x40E766E4F4A56727D8E766E4F4A56727, 0x7CDC1E0C7C7D68C69EDC1E0C7C7D68C6, 0x38919735E51578E554919735E51578E5, 0x4AAEFDD6DCD770412AAEFDD6DCD7704, 
    0x31C4488F3E8F96ED71C4488F3E8F96ED, 0xDFF3067B657990C37FF3067B657990C, 0x49B2B95E2F3F892FFDB2B95E2F3F892F, 0x7589C1B6A7E786CEBB89C1B6A7E786CE, 
    0x44C868FA4E3BC44EECC868FA4E3BC44E, 0x78F31012C6E3CBAFAAF31012C6E3CBAF, 0x3CBE992B5F8BDB8C60BE992B5F8BDB8C, 0x85E1C3D753D46D2685E1C3D753D46D, 
    0x5E3DCBB28C335E8CCE3DCBB28C335E8C, 0x6206B35A04EB516D8806B35A04EB516D, 0x264B3A639D83414E424B3A639D83414E, 0x1A70428B155B4EAF0470428B155B4EAF, 
    0x2B31EBC7FC870C2F5331EBC7FC870C2F, 0x170A932F745F03CE150A932F745F03CE, 0x53471A16ED3713EDDF471A16ED3713ED, 0x6F7C62FE65EF1C0C997C62FE65EF1C0C, 
    0x6BD68D2308226B088BD68D2308226B08, 0x57EDF5CB80FA64E9CDEDF5CB80FA64E9, 0x13A07CF2199274CA07A07CF2199274CA, 0x2F9B041A914A7B2B419B041A914A7B2B, 
    0x1EDAAD56789639AB16DAAD56789639AB, 0x22E1D5BEF04E364A50E1D5BEF04E364A, 0x66AC5C87692626699AAC5C8769262669, 0x5A97246FE1FE2988DC97246FE1FE2988, 
    0x42F0E1EBA9EA369342F0E1EBA9EA369, 0x381476F63246AC88721476F63246AC88, 0x7C59FFCFAB2EBCABB859FFCFAB2EBCAB, 0x4062872723F6B34AFE62872723F6B34A, 
    0x71232E6BCA2AF1CAA9232E6BCA2AF1CA, 0x4D18568342F2FE2BEF18568342F2FE2B, 0x955DFBADB9AEE082555DFBADB9AEE08, 0x356EA7525342E1E9636EA7525342E1E9, 
    0x6388911E7D1F2DDAE388911E7D1F2DDA, 0x5FB3E9F6F5C7223BA5B3E9F6F5C7223B, 0x1BFE60CF6CAF32186FFE60CF6CAF3218, 0x27C51827E4773DF929C51827E4773DF9, 
    0x1684B16B0DAB7F797E84B16B0DAB7F79, 0x2ABFC9838573709838BFC98385737098, 0x6EF240BA1C1B60BBF2F240BA1C1B60BB, 0x52C9385294C36F5AB4C9385294C36F5A, 
    0xC711223CFA3E5BB5C711223CFA3E5BB, 0x304A6ACB477BEA5A1A4A6ACB477BEA5A, 0x7407E3F2DE13FA79D007E3F2DE13FA79, 0x483C9B1A56CBF598963C9B1A56CBF598, 
    0x797D3256BF17B718C17D3256BF17B718, 0x45464ABE37CFB8F987464ABE37CFB8F9, 0x10BC387AEA7A8DA4D0BC387AEA7A8DA, 0x3D30BB6F267FA73B0B30BB6F267FA73B, 
    0x399A54B24BB2D03F199A54B24BB2D03F, 0x5A12C5AC36ADFDE5FA12C5AC36ADFDE, 0x41ECA5635A02CFFD95ECA5635A02CFFD, 0x7DD7DD8BD2DAC01CD3D7DD8BD2DAC01C, 
    0x4C9674C73B06829C849674C73B06829C, 0x70AD0C2FB3DE8D7DC2AD0C2FB3DE8D7D, 0x34E085162AB69D5E08E085162AB69D5E, 0x8DBFDFEA26E92BF4EDBFDFEA26E92BF, 
    0x5663D78FF90E185EA663D78FF90E185E, 0x6A58AF6771D617BFE058AF6771D617BF, 0x2E15265EE8BE079C2A15265EE8BE079C, 0x122E5EB66066087D6C2E5EB66066087D, 
    0x236FF7FA89BA4AFD3B6FF7FA89BA4AFD, 0x1F548F120162451C7D548F120162451C, 0x5B19062B980A553FB719062B980A553F, 0x67227EC310D25ADEF1227EC310D25ADE, 
    0x524CD9914390BB37924CD9914390BB37, 0x6E77A179CB48B4D6D477A179CB48B4D6, 0x2A3A28405220A4F51E3A28405220A4F5, 0x160150A8DAF8AB14580150A8DAF8AB14, 
    0x2740F9E43324E9940F40F9E43324E994, 0x1B7B810CBBFCE675497B810CBBFCE675, 0x5F3608352294F656833608352294F656, 0x630D70DDAA4CF9B7C50D70DDAA4CF9B7, 
    0x3DB55AACF12C73562DB55AACF12C7356, 0x18E224479F47CB76B8E224479F47CB7, 0x45C3AB7DE09C6C94A1C3AB7DE09C6C94, 0x79F8D39568446375E7F8D39568446375, 
    0x48B97AD9819821F5B0B97AD9819821F5, 0x7482023109402E14F682023109402E14, 0x30CF8B0890283E373CCF8B0890283E37, 0xCF4F3E018F031D67AF4F3E018F031D6, 
    0x85E1C3D753D46D2685E1C3D753D46D2, 0x346564D5FDE549332E6564D5FDE54933, 0x7028EDEC648D5910E428EDEC648D5910, 0x4C139504EC5556F1A2139504EC5556F1, 
    0x7D523C4805891471F5523C4805891471, 0x416944A08D511B90B36944A08D511B90, 0x524CD9914390BB37924CD9914390BB3, 0x391FB5719CE104523F1FB5719CE10452, 
    0x67A79F00C7818EB3D7A79F00C7818EB3, 0x5B9CE7E84F598152919CE7E84F598152, 0x1FD16ED1D63191715BD16ED1D6319171, 0x23EA16395EE99E901DEA16395EE99E90, 
    0x12ABBF75B735DC104AABBF75B735DC10, 0x2E90C79D3FEDD3F10C90C79D3FEDD3F1, 0x6ADD4EA4A685C3D2C6DD4EA4A685C3D2, 0x56E6364C2E5DCC3380E6364C2E5DCC33, 
    0x42F0E1EBA9EA369342F0E1EBA9EA3693, 0x7ECB99032132397204CB990321323972, 0x3A86103AB85A2951CE86103AB85A2951, 0x6BD68D2308226B088BD68D2308226B0, 
    0x37FCC19ED95E6430DFFCC19ED95E6430, 0xBC7B97651866BD199C7B97651866BD1, 0x4F8A304FC8EE7BF2538A304FC8EE7BF2, 0x73B148A74036741315B148A740367413, 
    0x2D0962D61B56FEF2FD0962D61B56FEF2, 0x11321A3E938EF113BB321A3E938EF113, 0x557F93070AE6E130717F93070AE6E130, 0x6944EBEF823EEED13744EBEF823EEED1, 
    0x580542A36BE2AC51600542A36BE2AC51, 0x643E3A4BE33AA3B0263E3A4BE33AA3B0, 0x2073B3727A52B393EC73B3727A52B393, 0x1C48CB9AF28ABC72AA48CB9AF28ABC72, 
    0x18E224479F47CB76B8E224479F47CB76, 0x24D95CAF179FC497FED95CAF179FC497, 0x6094D5968EF7D4B43494D5968EF7D4B4, 0x5CAFAD7E062FDB5572AFAD7E062FDB55, 
    0x6DEE0432EFF399D525EE0432EFF399D5, 0x51D57CDA672B963463D57CDA672B9634, 0x1598F5E3FE438617A998F5E3FE438617, 0x29A38D0B769B89F6EFA38D0B769B89F6, 
    0x771BA77A2DFB0317071BA77A2DFB0317, 0x4B20DF92A5230CF64120DF92A5230CF6, 0xF6D56AB3C4B1CD58B6D56AB3C4B1CD5, 0x33562E43B4931334CD562E43B4931334, 
    0x217870F5D4F51B49A17870F5D4F51B4, 0x3E2CFFE7D5975E55DC2CFFE7D5975E55, 0x7A6176DE4CFF4E76166176DE4CFF4E76, 0x465A0E36C4274197505A0E36C4274197, 
    0x7334A9649765A07E3334A9649765A07E, 0x4F0FD18C1FBDAF9F750FD18C1FBDAF9F, 0xB4258B586D5BFBCBF4258B586D5BFBC, 0x3779205D0E0DB05DF979205D0E0DB05D, 
    0x6388911E7D1F2DDAE388911E7D1F2DD, 0x3A03F1F96F09FD3CE803F1F96F09FD3C, 0x7E4E78C0F661ED1F224E78C0F661ED1F, 0x427500287EB9E2FE647500287EB9E2FE, 
    0x1CCD2A5925D9681F8CCD2A5925D9681F, 0x20F652B1AD0167FECAF652B1AD0167FE, 0x64BBDB88346977DD00BBDB88346977DD, 0x5880A360BCB1783C4680A360BCB1783C, 
    0x69C10A2C556D3ABC11C10A2C556D3ABC, 0x55FA72C4DDB5355D57FA72C4DDB5355D, 0x11B7FBFD44DD257E9DB7FBFD44DD257E, 0x2D8C8315CC052A9FDB8C8315CC052A9F, 
    0x29266CC8A1C85D9BC9266CC8A1C85D9B, 0x151D14202910527A8F1D14202910527A, 0x51509D19B078425945509D19B0784259, 0x6D6BE5F138A04DB8036BE5F138A04DB8, 
    0x5C2A4CBDD17C0F38542A4CBDD17C0F38, 0x6011345559A400D91211345559A400D9, 0x245CBD6CC0CC10FAD85CBD6CC0CC10FA, 0x1867C58448141F1B9E67C58448141F1B, 
    0x46DFEFF5137495FA76DFEFF5137495FA, 0x7AE4971D9BAC9A1B30E4971D9BAC9A1B, 0x3EA91E2402C48A38FAA91E2402C48A38, 0x29266CC8A1C85D9BC9266CC8A1C85D9, 
    0x33D3CF8063C0C759EBD3CF8063C0C759, 0xFE8B768EB18C8B8ADE8B768EB18C8B8, 0x4BA53E517270D89B67A53E517270D89B, 0x779E46B9FAA8D77A219E46B9FAA8D77A, 
    0x217870F5D4F51B49A17870F5D4F51B49, 0x1D43081D5C2D14A8E743081D5C2D14A8, 0x590E8124C545048B2D0E8124C545048B, 0x6535F9CC4D9D0B6A6B35F9CC4D9D0B6A, 
    0x54745080A44149EA3C745080A44149EA, 0x684F28682C99460B7A4F28682C99460B, 0x2C02A151B5F15628B002A151B5F15628, 0x1039D9B93D2959C9F639D9B93D2959C9, 
    0x4E81F3C86649D3281E81F3C86649D328, 0x72BA8B20EE91DCC958BA8B20EE91DCC9, 0x36F7021977F9CCEA92F7021977F9CCEA, 0xACC7AF1FF21C30BD4CC7AF1FF21C30B, 
    0x3B8DD3BD16FD818B838DD3BD16FD818B, 0x7B6AB559E258E6AC5B6AB559E258E6A, 0x43FB226C074D9E490FFB226C074D9E49, 0x7FC05A848F9591A849C05A848F9591A8, 
    0x7B6AB559E258E6AC5B6AB559E258E6AC, 0x4751CDB16A80E94D1D51CDB16A80E94D, 0x31C4488F3E8F96ED71C4488F3E8F96E, 0x3F273C607B30F68F91273C607B30F68F, 
    0xE66952C92ECB40FC666952C92ECB40F, 0x325DEDC41A34BBEE805DEDC41A34BBEE, 0x761064FD835CABCD4A1064FD835CABCD, 0x4A2B1C150B84A42C0C2B1C150B84A42C, 
    0x1493366450E42ECDE493366450E42ECD, 0x28A84E8CD83C212CA2A84E8CD83C212C, 0x6CE5C7B54154310F68E5C7B54154310F, 0x50DEBF5DC98C3EEE2EDEBF5DC98C3EEE, 
    0x619F161120507C6E799F161120507C6E, 0x5DA46EF9A888738F3FA46EF9A888738F, 0x19E9E7C031E063ACF5E9E7C031E063AC, 0x25D29F28B9386C4DB3D29F28B9386C4D, 
    0x10BC387AEA7A8DA4D0BC387AEA7A8DA4, 0x2C87409262A282459687409262A28245, 0x68CAC9ABFBCA92665CCAC9ABFBCA9266, 0x54F1B14373129D871AF1B14373129D87, 
    0x65B0180F9ACEDF074DB0180F9ACEDF07, 0x598B60E71216D0E60B8B60E71216D0E6, 0x1DC6E9DE8B7EC0C5C1C6E9DE8B7EC0C5, 0x21FD913603A6CF2487FD913603A6CF24, 
    0x7F45BB4758C645C56F45BB4758C645C5, 0x437EC3AFD01E4A24297EC3AFD01E4A24, 0x7334A9649765A07E3334A9649765A07, 0x3B08327EC1AE55E6A508327EC1AE55E6, 
    0xA499B3228721766F2499B3228721766, 0x3672E3DAA0AA1887B472E3DAA0AA1887, 0x723F6AE339C208A47E3F6AE339C208A4, 0x4E04120BB11A07453804120BB11A0745, 
    0x4AAEFDD6DCD770412AAEFDD6DCD77041, 0x7695853E540F7FA06C95853E540F7FA0, 0x32D80C07CD676F83A6D80C07CD676F83, 0xEE374EF45BF6062E0E374EF45BF6062, 
    0x3FA2DDA3AC6322E2B7A2DDA3AC6322E2, 0x399A54B24BB2D03F199A54B24BB2D03, 0x47D42C72BDD33D203BD42C72BDD33D20, 0x7BEF549A350B32C17DEF549A350B32C1, 
    0x25577EEB6E6BB82095577EEB6E6BB820, 0x196C0603E6B3B7C1D36C0603E6B3B7C1, 0x5D218F3A7FDBA7E219218F3A7FDBA7E2, 0x611AF7D2F703A8035F1AF7D2F703A803, 
    0x505B5E9E1EDFEA83085B5E9E1EDFEA83, 0x6C6026769607E5624E6026769607E562, 0x282DAF4F0F6FF541842DAF4F0F6FF541, 0x1416D7A787B7FAA0C216D7A787B7FAA0, 
];