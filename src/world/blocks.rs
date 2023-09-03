use std::collections::HashMap;

use crate::nbt::NBT;

enum_pack::flattenum! {
    Axis = {
        X,
        Y,
        Z,
    }

    Side = {
        True,
        False,
    }

    Persistent = {
        True,
        False,
    }

    Lit = {
        Lit,
        Unlit,
    }

    Powered = {
        True,
        False,
    }

    Waxed = {
        Waxed,
        Unwaxed,
    }

    Copper = {
        Copper
    }

    CopperType = Copper + {
        Exposed,
        Weathered,
        Oxidized,
    }

    SmeltableOre = Copper + {
        Iron,
        Gold,
    }

    OreType = SmeltableOre + {
        Coal,
        Lapis,
        Redstone(Lit),
        Diamond,
        Emerald,
    }

    SaplingType = {
        Oak,
        Birch,
        Spruce,
        Jungle,
        DarkOak,
        Acacia,
        Crimson,
        Warped,
    }

    WoodType = SaplingType + {
        Mangrove,
    }

    LeafType = WoodType + {
        Azalea,
        FloweringAzalea
    }

    DoorType = WoodType + {
        Iron
    }

    ButtonType = WoodType + {
        Stone,
        PolishedBlackstone
    }

    PressurePlateType = ButtonType + {
        Iron,
        Gold
    }

    Direction4 = {
        North,
        East,
        South,
        West,
    }

    Direction5 = Direction4 + {
        Down
    }

    Direction6 = Direction5 + {
        Up
    }

    UpDown = {
        Up,
        Down,
    }

    SlabHalf = UpDown + {
        Double
    }

    Count4 = {
        One,
        Two,
        Three,
        Four
    }

    Count8 = Count4 + {
        Five,
        Six,
        Seven,
        Eight,
    }

    Level1 = {
        Zero,
        One,
    }
    
    Level2 = Level1 + {
        Two,
    }
    
    Level3 = Level2 + {
        Three,
    }
        
    
    Level4 = Level3 + {
        Four,
    }
    
    Level5 = Level4 + {
        Five,
    }

    Level7 = Level5 + {
        Six,
        Seven,
    }

    Level8 = Level7 + {
        Eight
    }

    Level15 = Level8 + {
        Nine,
        Ten,
        Eleven,
        Twelve,
        Thirteen,
        Fourteen,
        Fifteen
    }

    Level24 = Level15 + {
        Sixteen,
        Seventeen,
        Eighteen,
        Nineteen,
        Twenty,
        TwentyOne,
        TwentyTwo,
        TwentyThree,
        TwentyFour
    }

    Level25 = Level24 + {
        TwentyFive
    }

    HingeSide = {
        Left,
        Right
    }

    ChestHalf = HingeSide + {
        Single,
    }

    Bloom = {
        True,
        False,
    }

    SculkSensorPhase = {
        Active,
        Cooldown,
        Inactive,
    }

    WaterLogged = {
        WaterLogged,
        Air,
    }

    CopperStage = {
        Clean,
        Exposed,
        Wethered,
        Oxidized,
    }

    AnvilDamage = {
        Clean,
        Chipped,
        Damaged,
    }

    CoralType = {
        Brain,
        Bubble,
        Fire,
        Horn,
        Tube,
    }

    Snowy = {
        True,
        False,
    }

    Half = {
        Upper,
        Lower,
    }

    DripstoneThickness = {
        TipMerge,
        Tip,
        Frustum,
        Middle,
        Base,
    }

    Shrieking = {
        True,
        False,
    }

    CanSummon = {
        True,
        False,
    }

    Hanging = {
        True,
        False,
    }

    BedEnd = {
        Foot,
        Head,
    }

    Open = {
        Open,
        Closed,
    }

    DyeColor = {
        White,
        Red,
        Orange,
        Yellow,
        Green,
        Lime,
        Blue,
        LightBlue,
        Cyan,
        Magenta,
        Pink,
        Purple,
        Brown,
        Gray,
        LightGray,
        Black,
    }
    
    DyeColorClear = DyeColor + {
        Clear
    }

    TallFlower = {
        TallGrass,
        LargeFern,
        TallSeagrass,
        Peony,
        Lilac,
        Rose,
        Sunflower,
    }

    Flower = SaplingType + {
        Dandelion,
        Poppy,
        BlueOrchid,
        Allium,
        AzureBluet,
        RedTulip,
        OrangeTulip,
        WhiteTulip,
        PinkTulip,
        OxeyeDaisy,
        Cornflower,
        LilyOfTheValley,
        WitherRose,
        BrownMushroom,
        RedMushroom,
        CrimsonFungus,
        WarpedFungus,
        Cactus,
        Fern,
        DeadBush,
    }

    StairShape = {
        InnerLeft,
        InnerRight,
        OuterLeft,
        OuterRight,
        Straight,
    }

    Shape = {
        Stair(Direction4, UpDown, StairShape, WaterLogged),
        Slab(SlabHalf, WaterLogged),
        Wall(Side, Side, Side, Side, Side, WaterLogged),
        Fence(Side, Side, Side, Side, WaterLogged),
    }

    ShapedBlock = Shape + {
        Block,
    }

    WallFace = {
        Ceiling,
        Wall,
        Floor
    }

    Attachment = WallFace + {
        DoubleWall
    }

    Stripped = {
        Stripped,
        Unstripped,
    }

    LogType = {
        Log,
        Wood,
    }

    Drag = {
        Up,//false
        Down,//true
    }

    Berries = {
        True,
        False,
    }

    InWall = {
        True,
        False,
    }

    Trapped = {
        True,
        False,
    }

    Attached = {
        True,
        False,
    }

    Unstable = {
        True,
        False,
    }

    Locked = {
        True,
        False,
    }

    Tilt = {
        Full,
        Partial,
        Unstable,
        None,
    }

    StraightRailShape = {
        AscendingEast,
        AscendingNorth,
        AscendingSouth,
        AscendingWest,
        NorthSouth,
        EastWest,

    }

    RailShape = StraightRailShape +  {
        NorthEast,
        NorthWest,
        SouthEast,
        SouthWest,
    }

    RedstoneConnection = {
        None,
        Side,
        Up,
    }

    Instrument = {
        Banjo,
        Basedrum,
        Bass,
        Bell,
        Bit,
        Chime,
        CowBell,
        Creeper,
        CustomHead,
        Digeridoo,
        Dragon,
        Flute,
        Guitar,
        Harp,
        Hat,
        IronXylophone,
        Piglin,
        Skeleton,
        Snare,
        WitherSkeleton,
        Xylophone,
        Zombie,
    }

    ComparatorMode = {
        Compare,
        Subtract,
    }

    HasBook = {
        True,
        False,
    }

    Extended = {
        True,
        False,
    }

    Inverted = {
        True,
        False,
    }

    PistonHeadLength = {
        Short,
        Long,
    }

    PistonType = {
        Sticky,
        Normal,
    }

    Age7Crop = {
        Wheat,
        Carrot,
        Potato,
        MelonStem,
        PumpkinStem,
    }

    Age3Crop = {
        Beetroot,
        NetherWart
    }

    Wet = {
        Wet,
        Dry
    }

    Red = {
        Normal,
        Red,
    }

    IceType = {
        Normal,
        Packed,
        Blue,
    }

    GrassType = {
        Grass,
        Mycelium,
        Podzol
    }

    Dead = {
        Dead,
        Alive,
    }

    CoralShape = {
        Coral(Direction6),
        Block,
        Fan(Direction6),
    }

    Record = {
        True,
        False,
    }

    Eye = {
        True,
        False,
    }

    Signal = {
        True,
        False,
    }

    Sign = {
        WallSign(Direction4),
        Sign(Level15),
    }

    Leaves = {
        None,
        Small,
        Large,
    }

    Soul = {
        Soul,
        Normal,
    }

    HasBottle = {
        True,
        False,
    }

    Disarmed = {
        True,
        False,
    }

    Occupied = {
        True,
        False,
    }

    Crops = {
        Crop7(Age7Crop, Level7),
        Crop3(Age3Crop, Level3),
        MelonStem(Direction4),
        PumpkinStem(Direction4),
    }

    Blocks = Crops + {
        Air,
        CaveAir,
        Dirt,
        GrassBlock(GrassType, Snowy),
        Grass,
        Bedrock,
        Stone(ShapedBlock),
        Cobblestone(ShapedBlock),
        MossyCobblestone(ShapedBlock),
        Deepslate(Axis),
        InfestedDeepslate(Axis),
        StoneBrick(ShapedBlock),
        MossyStoneBrick(ShapedBlock),
        SmoothStone(ShapedBlock),
        CrackedStoneBricks,
        CrackedDeepslateBricks,
        DeepslateBricks(ShapedBlock),
        CobbledDeepslate(ShapedBlock),
        ChiseledDeepslate,
        ChiseledStoneBrick,
        DeepslateTile(ShapedBlock),
        PolishedDeepslate(ShapedBlock),
        Blackstone(ShapedBlock),
        PolishedBlackstone(ShapedBlock),
        PolishedBlackstoneBrick(ShapedBlock),
        Sandstone(Red, ShapedBlock),
        Ore(OreType),
        DeepslateOre(OreType),
        RawOre(SmeltableOre),
        OreBlock(OreType),
        Gravel,
        Tuff,
        Lava(Level15),
        Water(Level15),
        GlowLichen(Side, Side, Side, Side, Side, Side, WaterLogged),
        Vine(Side, Side, Side, Side, Side),
        Sculk,
        SculkVein(Side, Side, Side, Side, Side, Side, WaterLogged),
        SculkCatalist(Bloom),
        SculkSensor(Level15, WaterLogged, SculkSensorPhase),
        SculkShrieker(CanSummon, WaterLogged, Shrieking),
        Chest(Trapped, ChestHalf, Direction4, WaterLogged),
        EnderChest(Direction4, WaterLogged),
        Spawner,
        Andesite(ShapedBlock),
        Granite(ShapedBlock),
        Diorite(ShapedBlock),
        PolishedAndesite(ShapedBlock),
        PolishedGranite(ShapedBlock),
        PolishedDiorite(ShapedBlock),
        InfestedStone,
        DripstoneBlock,
        PointedDripstone(UpDown, DripstoneThickness, WaterLogged),
        Flower(Flower),
        SmoothBasalt,

        Log(WoodType, LogType, Stripped, Axis),
        Sapling(SaplingType, Level1),
        Planks(WoodType, ShapedBlock),
        Leaves(LeafType, Level7, Persistent, WaterLogged),
        PressurePlate(PressurePlateType, Powered),
        Button(ButtonType, Powered),
        Door(DoorType, Open, Powered, HingeSide, Half, Direction4),
        Trapdoor(DoorType, WaterLogged, Powered, UpDown, Open, Direction4),
        FenceGate(WoodType, Open, Powered, Direction4, InWall),
        Sign(WoodType, Sign, WaterLogged),

        MangrovePropagule(Level3, Hanging, Level1, WaterLogged),//ignore age
        MangroveRoots(WaterLogged),
        MuddyMangroveRoots(Axis),
        HangingRoots(WaterLogged),
        Azalea,
        FloweringAzalea,
        WarpedRoots,
        SporeBlossom,

        IronBars(Side, Side, Side, Side, WaterLogged),
        Calcite,
        AmethystBlock,
        BuddingAmethyst,
        AmethystCluster(Level3, Direction6, WaterLogged),
        DirtPath,
        HayBlock(Axis),
        FlowerPot,
        Potted(Flower),
        CrackedDeepslateTiles,
        Bed(DyeColor, Direction4, BedEnd, Occupied),
        Torch(Soul, Direction5),
        RedstoneTorch(Direction5, Lit),
        Furnace(Direction4, Lit),
        BlastFurnace(Direction4, Lit),
        Smoker(Direction4, Lit),
        PolishedBasalt(Axis),
        WaterCauldron(Level3),
        Grindstone(WallFace, Direction4),
        Anvil(AnvilDamage, Direction4),
        Brick(ShapedBlock),
        Sand(Red),
        Ladder(WaterLogged, Direction4),
        Glass(DyeColorClear),
        GlassPane(DyeColorClear, Side, Side, Side, Side, WaterLogged),
        TintedGlass,
        Candle(DyeColorClear, Lit, Count4, WaterLogged),
        Terracotta(DyeColorClear),
        GlazedTerracotta(DyeColor, Direction4),
        Bell(Powered, Direction4, Attachment), //wall face is attachment point with some data omitted
        Wool(DyeColor),
        Carpet(DyeColor),
        Seagrass,
        TallFlower(TallFlower, Half),
        Clay,
        MagmaBlock,
        BubbleColumn(Drag),
        SugarCane(Level15),
        CaveVines(Berries, Level24),
        CaveVinesPlant(Berries, Level24),
        WeepingVines(Level25),
        WeepingVinesPlant,
        TwistingVines(Level25),
        TwistingVinesPlant,
        Obsidian,
        Pumpkin,
        Melon,
        BeeNest(Level5, Direction4),
        BeeHive(Level5, Direction4),
        MossBlock,
        BigDripleaf(Tilt, Direction4, WaterLogged),
        BigDripleafStem(Direction4, WaterLogged),
        SmallDripleaf(Half, Direction4, WaterLogged),
        Cobweb,
        Rail(RailShape, WaterLogged),
        ActivatorRail(StraightRailShape, WaterLogged),
        PoweredRail(StraightRailShape, WaterLogged),
        DetectorRail(StraightRailShape, WaterLogged),
        SeaPickle(Count4, WaterLogged),
        MossCarpet,
        NetherSprouts,
        Kelp(Level25),
        SlimeBlock,
        HoneyBlock,
        RedstoneWire(RedstoneConnection, RedstoneConnection, RedstoneConnection, RedstoneConnection, Level15),
        Observer(Powered, Direction6),
        TripwireHook(Powered, Attached, Direction4),
        Hopper(Direction5, Powered),
        RedstoneLamp(Lit),
        Tnt(Unstable),
        Dispenser(Direction6, Powered),
        Dropper(Direction6, Powered),
        NoteBlock(Powered, Level24, Instrument),
        Repeater(Count4, Direction4, Locked, Powered),
        Comparator(Direction4, ComparatorMode, Powered),
        Lectern(Direction4, Powered, HasBook),
        Piston(PistonType, Direction6, Extended),
        PistonHead(PistonType, Direction6, PistonHeadLength),
        Target(Level15),
        Lever(WallFace, Direction4, Powered),
        LightningRod(Direction6, Powered, WaterLogged),
        DaylightDetector(Inverted, Level15),
        BrownMushroomBlock(Side, Side, Side, Side, Side, Side),
        RedMushroomBlock(Side, Side, Side, Side, Side, Side),
        MushroomStem(Side, Side, Side, Side, Side, Side),
        Chain(Axis, WaterLogged),
        RootedDirt,
        Farmland(Level7),
        SoulSand,
        CopperBlock(CopperType, Waxed),
        NetheriteBlock,
        CutCopper(CopperType, Waxed, ShapedBlock),
        Composter(Level8),
        StoneCutter(Direction4),
        CrimsonNylium,
        WarpedNylium,
        NetherGoldOre,
        NetherQuartzOre,
        AncientDebris,
        Sponge(Wet),
        ChiseledSandstone(Red),
        CutSandstone(Red, ShapedBlock),
        CoarseDirt,
        Mud,
        SmoothSandstone(Red, ShapedBlock),
        SmoothQuartz(ShapedBlock),
        Quartz(ShapedBlock),
        CarvedPumpkin(Direction4),
        JackOLantern(Direction4),
        MudBrick(ShapedBlock),
        NetherBrick(ShapedBlock),
        RedNetherBrick(ShapedBlock),
        ChiseledNetherBrick,
        CrackedNetherBrick,
        Purpur(ShapedBlock),
        Prismarine(ShapedBlock),
        DarkPrismarine(ShapedBlock),
        PrismarineBrick(ShapedBlock),
        Netherrack,
        SnowBlock,
        Ice(IceType),
        PurpurPillar(Axis),
        Bookshelf,
        SoulSoil,
        Basalt(Axis),
        Glowstone,
        PackedMud,
        EndStoneBrick(ShapedBlock),
        EndStone,
        ReinforcedDeepslate,
        ChiseledQuartz,
        QuartzBricks,
        QuartzPillar(Axis),
        SeaLantern,
        WarpedWartBlock,
        NetherWartBlock,
        BoneBlock(Axis),
        Concrete(DyeColor),
        ConcretePowder(DyeColor),
        Coral(CoralType, Dead, CoralShape),
        GildedBlackstone,
        CryingObsidian,
        DriedKelpBlock,
        ChiseledPolishedBlackstone,
        CrackedPolishedBlackstoneBrick,
        ShulkerBox(DyeColorClear, Direction6),
        Snow(Count8),
        Jukebox(Record),
        InfestedCobblestone,
        InfestedStoneBricks,
        InfestedCrackedStoneBricks,
        InfestedMossyStoneBricks,
        InfestedChiseledStoneBricks,
        EndPortalFrame(Eye, Direction4),
        EnchantingTable,
        CraftingTable,
        CartographyTable,
        FletchingTable,
        SmithingTable,
        ChorusFlower(Level5),
        ChorusPlant(Side, Side, Side, Side, Side, Side),
        Scaffolding(Level7, Side, WaterLogged),
        Bamboo(Level1, Leaves),
        BambooSapling,
        EndRod(Direction6),
        OchreFroglight(Axis),
        VerdantFroglight(Axis),
        PearlescentFroglight(Axis),
        PlayerHead(Level15),
        ZombieHead(Level15),
        CreeperHead(Level15),
        DragonHead(Level15),
        SkeletonSkull(Level15),
        WitherSkeletonSkull(Level15),
        Banner(DyeColor, Level15),
        Loom(Direction4),
        Barrel(Open, Direction6),
        Campfire(Soul, Direction4, Signal, WaterLogged, Lit),
        Shroomlight,
        RespawnAnchor(Level4),
        Lodestone,
        HoneycombBlock,
        Lantern(Soul, Hanging, WaterLogged),
        LilyPad,
        Fire(Soul, Level15, Side, Side, Side, Side, Side),
        BrewingStand(HasBottle, HasBottle, HasBottle),
        PowderSnow,
        TurtleEgg(Level2, Count4),
        Beacon,
        Tripwire(Powered, Disarmed, Attached, Side, Side, Side, Side),
        KelpPlant,
        DragonEgg,
        SweetBerryBush(Level3),
    }
}

fn read_nbt_str<'a>(nbt: &'a HashMap<String, NBT>, field: &str) -> &'a str {
    match nbt.get(field) {
        Some(NBT::String(v)) => v.as_str(),
        None => panic!("missing {field} attribute {:?}", nbt),
        t => panic!("{field} attribute not string {:?}", t),
    }
}

fn read_nbt_string_boolean(nbt: &HashMap<String, NBT>, field: &str) -> bool {
    match read_nbt_str(nbt, field) {
        "true" => true,
        "false" => false,
        s => panic!("expected true or false got {s}. {field} in {nbt:?}")
    }
}

fn get_lit(nbt: &HashMap<String, NBT>) -> Lit {
    match read_nbt_string_boolean(nbt, "lit") {
        true => Lit::Lit,
        false => Lit::Unlit,
    }
}

fn get_0_to_1(nbt: &HashMap<String, NBT>, field: &str) -> Level1 {
    match read_nbt_str(nbt, field) {
        "0" => Level1::Zero,
        "1" => Level1::One,
        t => panic!("expected '0' to '1' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_3(nbt: &HashMap<String, NBT>, field: &str) -> Level3 {
    match read_nbt_str(nbt, field) {
        "0" => Level3::Zero,
        "1" => Level3::One,
        "2" => Level3::Two,
        "3" => Level3::Three,
        t => panic!("expected '0' to '3' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_2(nbt: &HashMap<String, NBT>, field: &str) -> Level2 {
    match read_nbt_str(nbt, field) {
        "0" => Level2::Zero,
        "1" => Level2::One,
        "2" => Level2::Two,
        t => panic!("expected '0' to '3' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_4(nbt: &HashMap<String, NBT>, field: &str) -> Level4 {
    match read_nbt_str(nbt, field) {
        "0" => Level4::Zero,
        "1" => Level4::One,
        "2" => Level4::Two,
        "3" => Level4::Three,
        "4" => Level4::Four,
        t => panic!("expected '0' to '3' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_5(nbt: &HashMap<String, NBT>, field: &str) -> Level5 {
    match read_nbt_str(nbt, field) {
        "0" => Level5::Zero,
        "1" => Level5::One,
        "2" => Level5::Two,
        "3" => Level5::Three,
        "4" => Level5::Four,
        "5" => Level5::Five,
        t => panic!("expected '0' to '3' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_7(nbt: &HashMap<String, NBT>, field: &str) -> Level7 {
    match read_nbt_str(nbt, field) {
        "0" => Level7::Zero,
        "1" => Level7::One,
        "2" => Level7::Two,
        "3" => Level7::Three,
        "4" => Level7::Four,
        "5" => Level7::Five,
        "6" => Level7::Six,
        "7" => Level7::Seven,
        t => panic!("expected '0' to '7' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_8(nbt: &HashMap<String, NBT>, field: &str) -> Level8 {
    match read_nbt_str(nbt, field) {
        "0" => Level8::Zero,
        "1" => Level8::One,
        "2" => Level8::Two,
        "3" => Level8::Three,
        "4" => Level8::Four,
        "5" => Level8::Five,
        "6" => Level8::Six,
        "7" => Level8::Seven,
        "8" => Level8::Eight,
        t => panic!("expected '0' to '7' found {} (Mojang why are these strings?)", t)
    }
}

fn get_1_to_8(nbt: &HashMap<String, NBT>, field: &str) -> Count8 {
    match read_nbt_str(nbt, field) {
        "1" => Count8::One,
        "2" => Count8::Two,
        "3" => Count8::Three,
        "4" => Count8::Four,
        "5" => Count8::Five,
        "6" => Count8::Six,
        "7" => Count8::Seven,
        "8" => Count8::Eight,
        t => panic!("expected '0' to '7' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_15(nbt: &HashMap<String, NBT>, field: &str) -> Level15 {
    match read_nbt_str(nbt, field) {
        "0" => Level15::Zero,
        "1" => Level15::One,
        "2" => Level15::Two,
        "3" => Level15::Three,
        "4" => Level15::Four,
        "5" => Level15::Five,
        "6" => Level15::Six,
        "7" => Level15::Seven,
        "8" => Level15::Eight,
        "9" => Level15::Nine,
        "10" => Level15::Ten,
        "11" => Level15::Eleven,
        "12" => Level15::Twelve,
        "13" => Level15::Thirteen,
        "14" => Level15::Fourteen,
        "15" => Level15::Fifteen,
        t => panic!("expected '0' to '15' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_24(nbt: &HashMap<String, NBT>, field: &str) -> Level24 {
    match read_nbt_str(nbt, field) {
        "0" => Level24::Zero,
        "1" => Level24::One,
        "2" => Level24::Two,
        "3" => Level24::Three,
        "4" => Level24::Four,
        "5" => Level24::Five,
        "6" => Level24::Six,
        "7" => Level24::Seven,
        "8" => Level24::Eight,
        "9" => Level24::Nine,
        "10" => Level24::Ten,
        "11" => Level24::Eleven,
        "12" => Level24::Twelve,
        "13" => Level24::Thirteen,
        "14" => Level24::Fourteen,
        "15" => Level24::Fifteen,
        "16" => Level24::Sixteen,
        "17" => Level24::Seventeen,
        "18" => Level24::Eighteen,
        "19" => Level24::Nineteen,
        "20" => Level24::Twenty,
        "21" => Level24::TwentyOne,
        "22" => Level24::TwentyTwo,
        "23" => Level24::TwentyThree,
        "24" => Level24::TwentyFour,
        t => panic!("expected '0' to '24' found {} (Mojang why are these strings?)", t)
    }
}

fn get_0_to_25(nbt: &HashMap<String, NBT>, field: &str) -> Level25 {
    match read_nbt_str(nbt, field) {
        "0" => Level25::Zero,
        "1" => Level25::One,
        "2" => Level25::Two,
        "3" => Level25::Three,
        "4" => Level25::Four,
        "5" => Level25::Five,
        "6" => Level25::Six,
        "7" => Level25::Seven,
        "8" => Level25::Eight,
        "9" => Level25::Nine,
        "10" => Level25::Ten,
        "11" => Level25::Eleven,
        "12" => Level25::Twelve,
        "13" => Level25::Thirteen,
        "14" => Level25::Fourteen,
        "15" => Level25::Fifteen,
        "16" => Level25::Sixteen,
        "17" => Level25::Seventeen,
        "18" => Level25::Eighteen,
        "19" => Level25::Nineteen,
        "20" => Level25::Twenty,
        "21" => Level25::TwentyOne,
        "22" => Level25::TwentyTwo,
        "23" => Level25::TwentyThree,
        "24" => Level25::TwentyFour,
        "25" => Level25::TwentyFive,
        t => panic!("expected '0' to '25' found {} (Mojang why are these strings?)", t)
    }
}

fn get_level15(nbt: &HashMap<String, NBT>) -> Level15 {
    get_0_to_15(nbt, "level")
}

fn get_level8(nbt: &HashMap<String, NBT>) -> Level8 {
    get_0_to_8(nbt, "level")
}

fn get_power(nbt: &HashMap<String, NBT>) -> Level15 {
    get_0_to_15(nbt, "power")
}

fn get_powered_from_power(nbt: &HashMap<String, NBT>) -> Powered {
    match get_0_to_15(nbt, "power") {
        Level15::Zero => Powered::False,
        _ => Powered::True,
    }
}

fn get_distance(nbt: &HashMap<String, NBT>) -> Level7 {
    get_0_to_7(nbt, "distance")
}

fn get_level3(nbt: &HashMap<String, NBT>) -> Level3 {
    get_0_to_3(nbt, "level")
}

fn get_honey_level(nbt: &HashMap<String, NBT>) -> Level5 {
    get_0_to_5(nbt, "honey_level")
}

fn get_stage1(nbt: &HashMap<String, NBT>) -> Level1 {
    get_0_to_1(nbt, "stage")
}

fn get_note(nbt: &HashMap<String, NBT>) -> Level24 {
    get_0_to_24(nbt, "note")
}

fn get_note_age25(nbt: &HashMap<String, NBT>) -> Level25 {
    get_0_to_25(nbt, "age")
}

fn get_hatch2(nbt: &HashMap<String, NBT>) -> Level2 {
    get_0_to_2(nbt, "hatch")
}

fn get_layers8(nbt: &HashMap<String, NBT>) -> Count8 {
    get_1_to_8(nbt, "layers")
}

fn get_side(nbt: &HashMap<String, NBT>, d: Direction6) -> Side {
    let v = match d {
        Direction6::North => "north",
        Direction6::East => "east",
        Direction6::South => "south",
        Direction6::West => "west",
        Direction6::Up => "up",
        Direction6::Down => "down",
    };
    match read_nbt_string_boolean(nbt, v) {
        true => Side::True,
        false => Side::False,
    }
}

fn get_wall_side(nbt: &HashMap<String, NBT>, d: Direction6) -> Side {
    let v = match d {
        Direction6::North => "north",
        Direction6::East => "east",
        Direction6::South => "south",
        Direction6::West => "west",
        Direction6::Up => "up",
        Direction6::Down => "down",
    };
    match read_nbt_str(nbt, v) {
        "tall" => Side::True,
        "low" => Side::True,
        "none" => Side::False,
        v => panic!("expected low, tall or none got {v}")
    }
}

fn get_waterlogged(nbt: &HashMap<String, NBT>) -> WaterLogged {
    match read_nbt_string_boolean(nbt, "waterlogged") {
        true => WaterLogged::WaterLogged,
        false => WaterLogged::Air,
    }
}

fn get_bloom(nbt: &HashMap<String, NBT>) -> Bloom {
    match read_nbt_string_boolean(nbt, "bloom") {
        true => Bloom::True,
        false => Bloom::False,
    }
}

fn get_sculk_sensor_phase(nbt: &HashMap<String, NBT>) -> SculkSensorPhase {
    match read_nbt_str(nbt, "sculk_sensor_phase") {
        "active" => SculkSensorPhase::Active,
        "cooldown" => SculkSensorPhase::Cooldown,
        "inactive" => SculkSensorPhase::Inactive,
        v => panic!("invalid sculk state {v}")
    }
}

fn get_leaves(nbt: &HashMap<String, NBT>) -> Leaves {
    match read_nbt_str(nbt, "leaves") {
        "none" => Leaves::None,
        "small" => Leaves::Small,
        "large" => Leaves::Large,
        v => panic!("invalid leaf state {v}")
    }
}

fn get_chest_half(nbt: &HashMap<String, NBT>) -> ChestHalf {
    match read_nbt_str(nbt, "type") {
        "single" => ChestHalf::Single,
        "left" => ChestHalf::Left,
        "right" => ChestHalf::Right,
        v => panic!("invalid chest half state {v}")
    }
}

fn get_direction4(nbt: &HashMap<String, NBT>, field: &str) -> Direction4 {
    match read_nbt_str(nbt, field) {
        "north" => Direction4::North,
        "east" => Direction4::East,
        "south" => Direction4::South,
        "west" => Direction4::West,
        v => panic!("invalid direction state {v}")
    }
}

fn get_facing4(nbt: &HashMap<String, NBT>) -> Direction4 {
    get_direction4(nbt, "facing")
}

fn get_direction5(nbt: &HashMap<String, NBT>, field: &str) -> Direction5 {
    match read_nbt_str(nbt, field) {
        "north" => Direction5::North,
        "east" => Direction5::East,
        "south" => Direction5::South,
        "west" => Direction5::West,
        "down" => Direction5::Down,
        v => panic!("invalid direction state {v}")
    }
}

fn get_facing5(nbt: &HashMap<String, NBT>) -> Direction5 {
    get_direction5(nbt, "facing")
}

fn get_direction6(nbt: &HashMap<String, NBT>, field: &str) -> Direction6 {
    match read_nbt_str(nbt, field) {
        "north" => Direction6::North,
        "east" => Direction6::East,
        "south" => Direction6::South,
        "west" => Direction6::West,
        "down" => Direction6::Down,
        "up" => Direction6::Up,
        v => panic!("invalid direction state {v}")
    }
}

fn get_facing6(nbt: &HashMap<String, NBT>) -> Direction6 {
    get_direction6(nbt, "facing")
}

fn get_vertical_direction(nbt: &HashMap<String, NBT>) -> UpDown {
    match read_nbt_str(nbt, "vertical_direction") {
        "up" => UpDown::Up,
        "down" => UpDown::Down,
        v => panic!("invalid virtical direction {v}")
    }
}

fn get_thickness(nbt: &HashMap<String, NBT>) -> DripstoneThickness {
    match read_nbt_str(nbt, "thickness") {
        "tip_merge" => DripstoneThickness::TipMerge,
        "tip" => DripstoneThickness::Tip,
        "frustum" => DripstoneThickness::Frustum,
        "middle" => DripstoneThickness::Middle,
        "base" => DripstoneThickness::Base,
        v => panic!("invalid thickness {v}")
    }
}

fn get_snowy(nbt: &HashMap<String, NBT>) -> Snowy {
    match read_nbt_string_boolean(nbt, "snowy") {
        true => Snowy::True,
        false => Snowy::False,
    }
}

fn get_axis(nbt: &HashMap<String, NBT>) -> Axis {
    match read_nbt_str(nbt, "axis") {
        "x" => Axis::X,
        "y" => Axis::Y,
        "z" => Axis::Z,
        t => panic!("expected x, y, or z found {}", t)
    }
}

fn get_half(nbt: &HashMap<String, NBT>) -> Half {
    match read_nbt_str(nbt, "half") {
        "upper" => Half::Upper,
        "lower" => Half::Lower,
        t => panic!("expected \"upper\" or \"lower\" found {}", t)
    }
}

fn get_stair_up_down(nbt: &HashMap<String, NBT>) -> UpDown {
    match read_nbt_str(nbt, "half") {
        "top" => UpDown::Up,
        "bottom" => UpDown::Down,
        t => panic!("expected \"top\" or \"bottom\" found {}", t)
    }
}

fn get_slab_half(nbt: &HashMap<String, NBT>) -> SlabHalf {
    match read_nbt_str(nbt, "type") {
        "top" => SlabHalf::Up,
        "bottom" => SlabHalf::Down,
        "double" => SlabHalf::Double,
        t => panic!("expected \"double\", \"top\" or \"bottom\" found {}", t)
    }
}

fn get_can_summon(nbt: &HashMap<String, NBT>) -> CanSummon {
    match read_nbt_string_boolean(nbt, "can_summon") {
        true => CanSummon::True,
        false => CanSummon::False,
    }
}

fn get_shrieking(nbt: &HashMap<String, NBT>) -> Shrieking {
    match read_nbt_string_boolean(nbt, "shrieking") {
        true => Shrieking::True,
        false => Shrieking::False,
    }
}

fn get_hanging(nbt: &HashMap<String, NBT>) -> Hanging {
    match read_nbt_string_boolean(nbt, "hanging") {
        true => Hanging::True,
        false => Hanging::False,
    }
}

fn get_persistent(nbt: &HashMap<String, NBT>) -> Persistent {
    match read_nbt_string_boolean(nbt, "persistent") {
        true => Persistent::True,
        false => Persistent::False,
    }
}

fn get_bed_end(nbt: &HashMap<String, NBT>) -> BedEnd {
    match read_nbt_str(nbt, "part") {
        "head" => BedEnd::Head,
        "foot" => BedEnd::Foot,
        t => panic!("expected \"head\" or \"foot\" found {}", t)
    }
}

fn get_powered(nbt: &HashMap<String, NBT>) -> Powered {
    match read_nbt_string_boolean(nbt, "powered") {
        true => Powered::True,
        false => Powered::False,
    }
}

fn get_triggered(nbt: &HashMap<String, NBT>) -> Powered {
    match read_nbt_string_boolean(nbt, "triggered") {
        true => Powered::True,
        false => Powered::False,
    }
}

fn get_hinge_side(nbt: &HashMap<String, NBT>) -> HingeSide {
    match read_nbt_str(nbt, "hinge") {
        "left" => HingeSide::Left,
        "right" => HingeSide::Right,
        v => panic!("invalid hinge state {v}")
    }
}

fn get_open(nbt: &HashMap<String, NBT>) -> Open {
    match read_nbt_str(nbt, "open") {
        "true" => Open::Open,
        "false" => Open::Closed,
        v => panic!("invalid open state {v}")
    }
}

fn get_wall_face(nbt: &HashMap<String, NBT>) -> WallFace {
    match read_nbt_str(nbt, "face") {
        "floor" => WallFace::Floor,
        "wall" => WallFace::Wall,
        "ceiling" => WallFace::Ceiling,
        v => panic!("invalid open state {v}")
    }
}

fn get_1_to_4(nbt: &HashMap<String, NBT>, field: &str) -> Count4 {
    match read_nbt_str(nbt, field) {
        "1" => Count4::One,
        "2" => Count4::Two,
        "3" => Count4::Three,
        "4" => Count4::Four,
        t => panic!("expected '1' to '4' found {} (Mojang why are these strings?)", t)
    }
}

fn get_candles(nbt: &HashMap<String, NBT>) -> Count4 {
    get_1_to_4(nbt, "candles")
}

fn get_pickles(nbt: &HashMap<String, NBT>) -> Count4 {
    get_1_to_4(nbt, "pickles")
}

fn get_delay(nbt: &HashMap<String, NBT>) -> Count4 {
    get_1_to_4(nbt, "delay")
}

fn get_eggs(nbt: &HashMap<String, NBT>) -> Count4 {
    get_1_to_4(nbt, "eggs")
}


fn get_attachment(nbt: &HashMap<String, NBT>) -> Attachment {
    match read_nbt_str(nbt, "attachment") {
        "ceiling" => Attachment::Ceiling,
        "floor" => Attachment::Floor,
        "single_wall" => Attachment::Wall,
        "double_wall" => Attachment::DoubleWall,
        t => panic!("expected attachment point found {}", t)
    }
}

fn get_drag(nbt: &HashMap<String, NBT>) -> Drag {
    match read_nbt_string_boolean(nbt, "drag") {
        true => Drag::Down,
        false => Drag::Up,
    }
}

fn get_berries(nbt: &HashMap<String, NBT>) -> Berries {
    match read_nbt_string_boolean(nbt, "berries") {
        true => Berries::True,
        false => Berries::True,
    }
}

fn get_in_wall(nbt: &HashMap<String, NBT>) -> InWall {
    match read_nbt_string_boolean(nbt, "in_wall") {
        true => InWall::True,
        false => InWall::True,
    }
}

fn get_tilt(nbt: &HashMap<String, NBT>) -> Tilt {
    match read_nbt_str(nbt, "tilt") {
        "full" => Tilt::Full,
        "partial" => Tilt::Partial,
        "unstable" => Tilt::Unstable,
        "none" => Tilt::None,
        t => panic!("expected attachment point found {}", t)
    }
}

fn get_rail_shape(nbt: &HashMap<String, NBT>) -> RailShape{
    match read_nbt_str(nbt, "shape") {
        "east_west" => RailShape::EastWest,
        "north_east" => RailShape::NorthEast,
        "north_south" => RailShape::NorthSouth,
        "north_west" => RailShape::NorthWest,
        "south_east" => RailShape::SouthEast,
        "south_west" => RailShape::SouthWest,
        "ascending_north" => RailShape::AscendingNorth,
        "ascending_east" => RailShape::AscendingEast,
        "ascending_south" => RailShape::AscendingSouth,
        "ascending_west" => RailShape::AscendingWest,
        t => panic!("expected attachment point found {}", t)
    }
}

fn get_straight_rail_shape(nbt: &HashMap<String, NBT>) -> StraightRailShape {
    match read_nbt_str(nbt, "shape") {
        "east_west" => StraightRailShape::EastWest,
        "north_south" => StraightRailShape::NorthSouth,
        "ascending_north" => StraightRailShape::AscendingNorth,
        "ascending_east" => StraightRailShape::AscendingEast,
        "ascending_south" => StraightRailShape::AscendingSouth,
        "ascending_west" => StraightRailShape::AscendingWest,
        t => panic!("expected attachment point found {}", t)
    }
}

fn get_redstone_connection(nbt: &HashMap<String, NBT>, d: Direction4) -> RedstoneConnection {
    let v = match d {
        Direction4::North => "north",
        Direction4::East => "east",
        Direction4::South => "south",
        Direction4::West => "west",
    };
    match read_nbt_str(nbt, v) {
        "none" => RedstoneConnection::None,
        "side" => RedstoneConnection::Side,
        "up" => RedstoneConnection::Up,
        t => panic!("expected attachment point found {}", t)
    }
}

fn get_attached(nbt: &HashMap<String, NBT>) -> Attached {
    match read_nbt_string_boolean(nbt, "attached") {
        true => Attached::True,
        false => Attached::False,
    }
}

fn get_enabled(nbt: &HashMap<String, NBT>) -> Powered {
    match read_nbt_string_boolean(nbt, "enabled") {
        true => Powered::False,
        false => Powered::True,
    }
}

fn get_unstable(nbt: &HashMap<String, NBT>) -> Unstable {
    match read_nbt_string_boolean(nbt, "unstable") {
        true => Unstable::True,
        false => Unstable::False,
    }
}

fn get_locked(nbt: &HashMap<String, NBT>) -> Locked {
    match read_nbt_string_boolean(nbt, "locked") {
        true => Locked::True,
        false => Locked::False,
    }
}

fn get_instrument(nbt: &HashMap<String, NBT>) -> Instrument {
    match read_nbt_str(nbt, "instrument") {
        "banjo" => Instrument::Banjo,
        "basedrum" => Instrument::Basedrum,
        "bass" => Instrument::Bass,
        "bell" => Instrument::Bell,
        "bit" => Instrument::Bit,
        "chime" => Instrument::Chime,
        "cow_bell" => Instrument::CowBell,
        "creeper" => Instrument::Creeper,
        "custom_head" => Instrument::CustomHead,
        "digeridoo" => Instrument::Digeridoo,
        "dragon" => Instrument::Dragon,
        "flute" => Instrument::Flute,
        "guitar" => Instrument::Guitar,
        "harp" => Instrument::Harp,
        "hat" => Instrument::Hat,
        "iron_xylophone" => Instrument::IronXylophone,
        "piglin" => Instrument::Piglin,
        "skeleton" => Instrument::Skeleton,
        "snare" => Instrument::Snare,
        "wither_skeleton" => Instrument::WitherSkeleton,
        "xylophone" => Instrument::Xylophone,
        "zombie" => Instrument::Zombie,
        t => panic!("expected instrument point found {}", t)
    }
}

fn get_has_book(nbt: &HashMap<String, NBT>) -> HasBook {
    match read_nbt_string_boolean(nbt, "has_book") {
        true => HasBook::True,
        false => HasBook::False,
    }
}

fn get_comparator_mode(nbt: &HashMap<String, NBT>) -> ComparatorMode {
    match read_nbt_str(nbt, "mode") {
        "compare" => ComparatorMode::Compare,
        "subtract" => ComparatorMode::Subtract,
        t => panic!("expected comparator mode point found {}", t)
    }
}

fn get_extended(nbt: &HashMap<String, NBT>) -> Extended {
    match read_nbt_string_boolean(nbt, "extended") {
        true => Extended::True,
        false => Extended::False,
    }
}

fn get_head_length(nbt: &HashMap<String, NBT>) -> PistonHeadLength {
    match read_nbt_string_boolean(nbt, "short") {
        true => PistonHeadLength::Short,
        false => PistonHeadLength::Long,
    }
}

fn get_inverted(nbt: &HashMap<String, NBT>) -> Inverted {
    match read_nbt_string_boolean(nbt, "inverted") {
        true => Inverted::True,
        false => Inverted::False,
    }
}

fn get_piston_type(nbt: &HashMap<String, NBT>) -> PistonType {
    match read_nbt_str(nbt, "type") {
        "sticky" => PistonType::Sticky,
        "normal" => PistonType::Normal,
        t => panic!("expected piston type found {}", t)
    }
}

fn get_moisture(nbt: &HashMap<String, NBT>) -> Level7 {
    get_0_to_7(nbt, "moisture")
}

fn get_age1(nbt: &HashMap<String, NBT>) -> Level1 {
    get_0_to_1(nbt, "age")
}

fn get_age3(nbt: &HashMap<String, NBT>) -> Level3 {
    get_0_to_3(nbt, "age")
}

fn get_age5(nbt: &HashMap<String, NBT>) -> Level5 {
    get_0_to_5(nbt, "age")
}

fn get_age7(nbt: &HashMap<String, NBT>) -> Level7 {
    get_0_to_7(nbt, "age")
}

fn get_age15(nbt: &HashMap<String, NBT>) -> Level15 {
    get_0_to_15(nbt, "age")
}

fn get_age24(nbt: &HashMap<String, NBT>) -> Level24 {
    get_0_to_24(nbt, "age")
}

fn get_age25(nbt: &HashMap<String, NBT>) -> Level25 {
    get_0_to_25(nbt, "age")
}


fn get_charges4(nbt: &HashMap<String, NBT>) -> Level4 {
    get_0_to_4(nbt, "charges")
}

fn get_record(nbt: &HashMap<String, NBT>) -> Record {
    match read_nbt_string_boolean(nbt, "has_record") {
        true => Record::True,
        false => Record::False,
    }
}

fn get_eye(nbt: &HashMap<String, NBT>) -> Eye {
    match read_nbt_string_boolean(nbt, "eye") {
        true => Eye::True,
        false => Eye::False,
    }
}

fn get_signal(nbt: &HashMap<String, NBT>) -> Signal {
    match read_nbt_string_boolean(nbt, "signal_fire") {
        true => Signal::True,
        false => Signal::False,
    }
}

fn get_down(nbt: &HashMap<String, NBT>) -> Side {
    match read_nbt_string_boolean(nbt, "bottom") {
        true => Side::True,
        false => Side::False,
    }
}

fn get_stair_shape(nbt: &HashMap<String, NBT>) -> StairShape {
    match read_nbt_str(nbt, "shape") {
        "inner_left" => StairShape::InnerLeft,
        "inner_right" => StairShape::InnerRight,
        "outer_left" => StairShape::OuterLeft,
        "outer_right" => StairShape::OuterRight,
        "straight" => StairShape::Straight,
        t => panic!("expected stair shape found {}", t)
    }
}

fn get_stair(nbt: &HashMap<String, NBT>) -> ShapedBlock {
    ShapedBlock::Stair(get_facing4(nbt), get_stair_up_down(nbt), get_stair_shape(nbt), get_waterlogged(nbt))
}

fn get_slab(nbt: &HashMap<String, NBT>) -> ShapedBlock {
    ShapedBlock::Slab(get_slab_half(nbt), get_waterlogged(nbt))
}

fn get_fence(nbt: &HashMap<String, NBT>) -> ShapedBlock {
    ShapedBlock::Fence(get_side(nbt, Direction6::North),
        get_side(nbt, Direction6::East),
        get_side(nbt, Direction6::South),
        get_side(nbt, Direction6::West),
        get_waterlogged(nbt))
}

fn get_wall(nbt: &HashMap<String, NBT>) -> ShapedBlock {
    ShapedBlock::Wall(
        get_wall_side(nbt, Direction6::North),
        get_wall_side(nbt, Direction6::East),
        get_wall_side(nbt, Direction6::South),
        get_wall_side(nbt, Direction6::West),
        get_wall_side(nbt, Direction6::Up),
        get_waterlogged(nbt))
}

fn get_glass_pane(col: DyeColorClear, nbt: &HashMap<String, NBT>) -> Blocks {
    Blocks::GlassPane(
        col,
        get_side(nbt, Direction6::North),
        get_side(nbt, Direction6::East),
        get_side(nbt, Direction6::South),
        get_side(nbt, Direction6::West),
        get_waterlogged(nbt))
}

fn get_rotation(nbt: &HashMap<String, NBT>) -> Level15 {
    get_0_to_15(nbt, "rotation")
}

fn get_bottle(nbt: &HashMap<String, NBT>, bottle: u8) -> HasBottle {
    match read_nbt_string_boolean(nbt, &format!("has_bottle_{}", bottle)) {
        true => HasBottle::True,
        false => HasBottle::False,
    }
}

fn get_disarmed(nbt: &HashMap<String, NBT>) -> Disarmed {
    match read_nbt_string_boolean(nbt, "disarmed") {
        true => Disarmed::True,
        false => Disarmed::False,
    }
}

fn get_occupied(nbt: &HashMap<String, NBT>) -> Occupied {
    match read_nbt_string_boolean(nbt, "shape") {
        true => Occupied::True,
        false => Occupied::False,
    }
}

pub fn string_to_block(name: &str, nbt: Option<&HashMap<String, NBT>>) -> Blocks {
    match nbt {
        None => {
            match name {
                "minecraft:air" => Blocks::Air,
                "minecraft:cave_air" => Blocks::CaveAir,
                "minecraft:dirt" => Blocks::Dirt,
                "minecraft:grass" => Blocks::Grass,
                "minecraft:bedrock" => Blocks::Bedrock,
                "minecraft:tuff" => Blocks::Tuff,
                "minecraft:gravel" => Blocks::Gravel,

                "minecraft:deepslate_gold_ore" => Blocks::DeepslateOre(OreType::Gold),
                "minecraft:deepslate_iron_ore" => Blocks::DeepslateOre(OreType::Iron),
                "minecraft:deepslate_diamond_ore" => Blocks::DeepslateOre(OreType::Diamond),
                "minecraft:deepslate_coal_ore" => Blocks::DeepslateOre(OreType::Coal),
                "minecraft:deepslate_emerald_ore" => Blocks::DeepslateOre(OreType::Emerald),
                "minecraft:deepslate_lapis_ore" => Blocks::DeepslateOre(OreType::Lapis),
                "minecraft:deepslate_copper_ore" => Blocks::DeepslateOre(OreType::Copper),

                "minecraft:gold_ore" => Blocks::Ore(OreType::Gold),
                "minecraft:iron_ore" => Blocks::Ore(OreType::Iron),
                "minecraft:diamond_ore" => Blocks::Ore(OreType::Diamond),
                "minecraft:coal_ore" => Blocks::Ore(OreType::Coal),
                "minecraft:emerald_ore" => Blocks::Ore(OreType::Emerald),
                "minecraft:lapis_ore" => Blocks::Ore(OreType::Lapis),
                "minecraft:copper_ore" => Blocks::Ore(OreType::Copper),

                "minecraft:raw_gold_block" => Blocks::RawOre(SmeltableOre::Gold),
                "minecraft:raw_iron_block" => Blocks::RawOre(SmeltableOre::Iron),
                "minecraft:raw_copper_block" => Blocks::RawOre(SmeltableOre::Copper),

                "minecraft:gold_block" => Blocks::OreBlock(OreType::Gold),
                "minecraft:iron_block" => Blocks::OreBlock(OreType::Iron),
                "minecraft:diamond_block" => Blocks::OreBlock(OreType::Diamond),
                "minecraft:coal_block" => Blocks::OreBlock(OreType::Coal),
                "minecraft:emerald_block" => Blocks::OreBlock(OreType::Emerald),
                "minecraft:lapis_block" => Blocks::OreBlock(OreType::Lapis),
                
                "minecraft:sculk" => Blocks::Sculk,
                "minecraft:mossy_cobblestone" => Blocks::MossyCobblestone(ShapedBlock::Block),
                "minecraft:cobblestone" => Blocks::Cobblestone(ShapedBlock::Block),
                "minecraft:spawner" => Blocks::Spawner,
                "minecraft:andesite" => Blocks::Andesite(ShapedBlock::Block),
                "minecraft:granite" => Blocks::Granite(ShapedBlock::Block),
                "minecraft:diorite" => Blocks::Diorite(ShapedBlock::Block),
                "minecraft:polished_andesite" => Blocks::PolishedAndesite(ShapedBlock::Block),
                "minecraft:polished_granite" => Blocks::PolishedGranite(ShapedBlock::Block),
                "minecraft:polished_diorite" => Blocks::PolishedDiorite(ShapedBlock::Block),
                "minecraft:stone" => Blocks::Stone(ShapedBlock::Block),
                "minecraft:redstone_block" => Blocks::OreBlock(OreType::Redstone(Lit::Unlit)),
                "minecraft:infested_stone" => Blocks::InfestedStone,
                "minecraft:dripstone_block" => Blocks::DripstoneBlock,
                "minecraft:cracked_deepslate_bricks" => Blocks::CrackedDeepslateBricks,
                "minecraft:deepslate_bricks" => Blocks::DeepslateBricks(ShapedBlock::Block),
                "minecraft:cobbled_deepslate" => Blocks::CobbledDeepslate(ShapedBlock::Block),
                "minecraft:chiseled_deepslate" => Blocks::ChiseledDeepslate,
                "minecraft:smooth_basalt" => Blocks::SmoothBasalt,
                "minecraft:soul_fire" => {
                    Blocks::Fire(Soul::Soul,
                        Level15::Zero,
                        Side::True,
                        Side::True,
                        Side::True,
                        Side::True,
                        Side::True)
                }

                "minecraft:oak_planks" => Blocks::Planks(WoodType::Oak, ShapedBlock::Block),
                "minecraft:birch_planks" => Blocks::Planks(WoodType::Birch, ShapedBlock::Block),
                "minecraft:spruce_planks" => Blocks::Planks(WoodType::Spruce, ShapedBlock::Block),
                "minecraft:jungle_planks" => Blocks::Planks(WoodType::Jungle, ShapedBlock::Block),
                "minecraft:dark_oak_planks" => Blocks::Planks(WoodType::DarkOak, ShapedBlock::Block),
                "minecraft:acacia_planks" => Blocks::Planks(WoodType::Acacia, ShapedBlock::Block),
                "minecraft:crimson_planks" => Blocks::Planks(WoodType::Crimson, ShapedBlock::Block),
                "minecraft:warped_planks" => Blocks::Planks(WoodType::Warped, ShapedBlock::Block),
                "minecraft:mangrove_planks" => Blocks::Planks(WoodType::Mangrove, ShapedBlock::Block),

                "minecraft:copper_block" => Blocks::CopperBlock(CopperType::Copper, Waxed::Unwaxed),
                "minecraft:exposed_copper" => Blocks::CopperBlock(CopperType::Exposed, Waxed::Unwaxed),
                "minecraft:weathered_copper" => Blocks::CopperBlock(CopperType::Weathered, Waxed::Unwaxed),
                "minecraft:oxidized_copper" => Blocks::CopperBlock(CopperType::Oxidized, Waxed::Unwaxed),

                "minecraft:waxed_copper_block" => Blocks::CopperBlock(CopperType::Copper, Waxed::Waxed),
                "minecraft:waxed_exposed_copper" => Blocks::CopperBlock(CopperType::Exposed, Waxed::Waxed),
                "minecraft:waxed_weathered_copper" => Blocks::CopperBlock(CopperType::Weathered, Waxed::Waxed),
                "minecraft:waxed_oxidized_copper" => Blocks::CopperBlock(CopperType::Oxidized, Waxed::Waxed),

                "minecraft:cut_copper" => Blocks::CutCopper(CopperType::Copper, Waxed::Unwaxed, ShapedBlock::Block),
                "minecraft:exposed_cut_copper" => Blocks::CutCopper(CopperType::Exposed, Waxed::Unwaxed, ShapedBlock::Block),
                "minecraft:weathered_cut_copper" => Blocks::CutCopper(CopperType::Weathered, Waxed::Unwaxed, ShapedBlock::Block),
                "minecraft:oxidized_cut_copper" => Blocks::CutCopper(CopperType::Oxidized, Waxed::Unwaxed, ShapedBlock::Block),
                "minecraft:waxed_cut_copper" => Blocks::CutCopper(CopperType::Copper, Waxed::Waxed, ShapedBlock::Block),
                "minecraft:waxed_exposed_cut_copper" => Blocks::CutCopper(CopperType::Exposed, Waxed::Waxed, ShapedBlock::Block),
                "minecraft:waxed_weathered_cut_copper" => Blocks::CutCopper(CopperType::Weathered, Waxed::Waxed, ShapedBlock::Block),
                "minecraft:waxed_oxidized_cut_copper" => Blocks::CutCopper(CopperType::Oxidized, Waxed::Waxed, ShapedBlock::Block),

                "minecraft:deepslate_tiles" => Blocks::DeepslateTile(ShapedBlock::Block),
                "minecraft:polished_deepslate" => Blocks::PolishedDeepslate(ShapedBlock::Block),
                "minecraft:calcite" => Blocks::Calcite,
                "minecraft:amethyst_block" => Blocks::AmethystBlock,
                "minecraft:budding_amethyst" => Blocks::BuddingAmethyst,
                "minecraft:dirt_path" => Blocks::DirtPath,
                "minecraft:cracked_deepslate_tiles" => Blocks::CrackedDeepslateTiles,
                "minecraft:smooth_stone" => Blocks::SmoothStone(ShapedBlock::Block),
                "minecraft:moss_block" => Blocks::MossBlock,
                "minecraft:cobweb" => Blocks::Cobweb,
                "minecraft:rooted_dirt" => Blocks::RootedDirt,
                "minecraft:nether_gold_ore" => Blocks::NetherGoldOre,
                "minecraft:nether_quartz_ore" => Blocks::NetherQuartzOre,
                "minecraft:ancient_debris" => Blocks::AncientDebris,
                "minecraft:netherite_block" => Blocks::NetheriteBlock,
                "minecraft:coarse_dirt" => Blocks::CoarseDirt,
                "minecraft:mud" => Blocks::Mud,

                "minecraft:flowering_azalea" => Blocks::FloweringAzalea,
                "minecraft:azalea" => Blocks::Azalea,

                "minecraft:flower_pot" => Blocks::FlowerPot,
                "minecraft:dandelion" => Blocks::Flower(Flower::Dandelion),
                "minecraft:potted_dandelion" => Blocks::Potted(Flower::Dandelion),

                "minecraft:poppy" => Blocks::Flower(Flower::Poppy),
                "minecraft:potted_poppy" => Blocks::Potted(Flower::Poppy),

                "minecraft:blue_orchid" => Blocks::Flower(Flower::BlueOrchid),
                "minecraft:potted_blue_orchid" => Blocks::Potted(Flower::BlueOrchid),

                "minecraft:allium" => Blocks::Flower(Flower::Allium),
                "minecraft:potted_allium" => Blocks::Potted(Flower::Allium),

                "minecraft:azure_bluet" => Blocks::Flower(Flower::AzureBluet),
                "minecraft:potted_azure_bluet" => Blocks::Potted(Flower::AzureBluet),

                "minecraft:red_tulip" => Blocks::Flower(Flower::RedTulip),
                "minecraft:potted_red_tulip" => Blocks::Potted(Flower::RedTulip),

                "minecraft:orange_tulip" => Blocks::Flower(Flower::OrangeTulip),
                "minecraft:potted_orange_tulip" => Blocks::Potted(Flower::OrangeTulip),

                "minecraft:white_tulip" => Blocks::Flower(Flower::WhiteTulip),
                "minecraft:potted_white_tulip" => Blocks::Potted(Flower::WhiteTulip),

                "minecraft:pink_tulip" => Blocks::Flower(Flower::PinkTulip),
                "minecraft:potted_pink_tulip" => Blocks::Potted(Flower::PinkTulip),

                "minecraft:oxeye_daisy" => Blocks::Flower(Flower::OxeyeDaisy),
                "minecraft:potted_oxeye_daisy" => Blocks::Potted(Flower::OxeyeDaisy),

                "minecraft:cornflower" => Blocks::Flower(Flower::Cornflower),
                "minecraft:potted_cornflower" => Blocks::Potted(Flower::Cornflower),

                "minecraft:lily_of_the_valley" => Blocks::Flower(Flower::LilyOfTheValley),
                "minecraft:potted_lily_of_the_valley" => Blocks::Potted(Flower::LilyOfTheValley),

                "minecraft:wither_rose" => Blocks::Flower(Flower::WitherRose),
                "minecraft:potted_wither_rose" => Blocks::Potted(Flower::WitherRose),

                "minecraft:red_mushroom" => Blocks::Flower(Flower::RedMushroom),
                "minecraft:potted_red_mushroom" => Blocks::Potted(Flower::RedMushroom),

                "minecraft:brown_mushroom" => Blocks::Flower(Flower::BrownMushroom),
                "minecraft:potted_brown_mushroom" => Blocks::Potted(Flower::BrownMushroom),
                
                "minecraft:crimson_fungus" => Blocks::Flower(Flower::CrimsonFungus),
                "minecraft:potted_crimson_fungus" => Blocks::Potted(Flower::CrimsonFungus),

                "minecraft:warped_fungus" => Blocks::Flower(Flower::WarpedFungus),
                "minecraft:potted_warped_fungus" => Blocks::Potted(Flower::WarpedFungus),

                "minecraft:fern" => Blocks::Flower(Flower::Fern),
                "minecraft:potted_fern" => Blocks::Potted(Flower::Fern),

                "minecraft:dead_bush" => Blocks::Flower(Flower::DeadBush),
                "minecraft:potted_dead_bush" => Blocks::Potted(Flower::DeadBush),

                "minecraft:potted_cactus" => Blocks::Potted(Flower::Cactus),

                "minecraft:torch" => Blocks::Torch(Soul::Normal, Direction5::Down),
                "minecraft:soul_torch" => Blocks::Torch(Soul::Soul, Direction5::Down),
                "minecraft:bricks" => Blocks::Brick(ShapedBlock::Block),
                "minecraft:sand" => Blocks::Sand(Red::Normal),
                "minecraft:red_sand" => Blocks::Sand(Red::Red),
                "minecraft:seagrass" => Blocks::Seagrass,
                "minecraft:clay" => Blocks::Clay,
                "minecraft:magma_block" => Blocks::MagmaBlock,
                "minecraft:obsidian" => Blocks::Obsidian,
                "minecraft:pumpkin" => Blocks::Pumpkin,
                "minecraft:melon" => Blocks::Melon,
                "minecraft:moss_carpet" => Blocks::MossCarpet,
                "minecraft:nether_sprouts" => Blocks::NetherSprouts,
                "minecraft:warped_roots" => Blocks::WarpedRoots,
                "minecraft:spore_blossom" => Blocks::SporeBlossom,
                "minecraft:slime_block" => Blocks::SlimeBlock,
                "minecraft:honey_block" => Blocks::HoneyBlock,
                "minecraft:twisting_vines_plant" => Blocks::TwistingVinesPlant,
                "minecraft:weeping_vines_plant" => Blocks::WeepingVinesPlant,
                "minecraft:sandstone" => Blocks::Sandstone(Red::Normal, ShapedBlock::Block),
                "minecraft:red_sandstone" => Blocks::Sandstone(Red::Red, ShapedBlock::Block),
                "minecraft:soul_sand" => Blocks::SoulSand,
                "minecraft:crimson_nylium" => Blocks::CrimsonNylium,
                "minecraft:warped_nylium" => Blocks::WarpedNylium,
                "minecraft:sponge" => Blocks::Sponge(Wet::Dry),
                "minecraft:wet_sponge" => Blocks::Sponge(Wet::Wet),
                "minecraft:chiseled_sandstone" => Blocks::ChiseledSandstone(Red::Normal),
                "minecraft:chiseled_red_sandstone" => Blocks::ChiseledSandstone(Red::Red),
                "minecraft:cut_sandstone" => Blocks::CutSandstone(Red::Normal, ShapedBlock::Block),
                "minecraft:cut_red_sandstone" => Blocks::CutSandstone(Red::Red, ShapedBlock::Block),
                "minecraft:smooth_sandstone" => Blocks::SmoothSandstone(Red::Normal, ShapedBlock::Block),
                "minecraft:smooth_red_sandstone" => Blocks::SmoothSandstone(Red::Red, ShapedBlock::Block),
                "minecraft:smooth_quartz" => Blocks::SmoothQuartz(ShapedBlock::Block),
                "minecraft:quartz_block" => Blocks::Quartz(ShapedBlock::Block),
                "minecraft:netherrack" => Blocks::Netherrack,
                "minecraft:snow_block" => Blocks::SnowBlock,
                "minecraft:ice" => Blocks::Ice(IceType::Normal),
                "minecraft:packed_ice" => Blocks::Ice(IceType::Packed),
                "minecraft:blue_ice" => Blocks::Ice(IceType::Blue),
                "minecraft:purpur_block" => Blocks::Purpur(ShapedBlock::Block),
                "minecraft:bookshelf" => Blocks::Bookshelf,
                "minecraft:soul_soil" => Blocks::SoulSoil,
                "minecraft:glowstone" => Blocks::Glowstone,
                "minecraft:stone_bricks" => Blocks::StoneBrick(ShapedBlock::Block),
                "minecraft:mossy_stone_bricks" => Blocks::MossyStoneBrick(ShapedBlock::Block),
                "minecraft:cracked_stone_bricks" => Blocks::CrackedStoneBricks,
                "minecraft:chiseled_stone_bricks" => Blocks::ChiseledStoneBrick,
                "minecraft:packed_mud" => Blocks::PackedMud,
                "minecraft:mud_bricks" => Blocks::MudBrick(ShapedBlock::Block),
                "minecraft:end_stone_bricks" => Blocks::EndStoneBrick(ShapedBlock::Block),
                "minecraft:end_stone" => Blocks::EndStone,
                "minecraft:nether_bricks" => Blocks::NetherBrick(ShapedBlock::Block),
                "minecraft:chiseled_nether_bricks" => Blocks::ChiseledNetherBrick,
                "minecraft:cracked_nether_bricks" => Blocks::CrackedNetherBrick,
                "minecraft:reinforced_deepslate" => Blocks::ReinforcedDeepslate,
                "minecraft:chiseled_quartz_block" => Blocks::ChiseledQuartz,
                "minecraft:quartz_bricks" => Blocks::QuartzBricks,
                "minecraft:prismarine" => Blocks::Prismarine(ShapedBlock::Block),
                "minecraft:prismarine_bricks" => Blocks::PrismarineBrick(ShapedBlock::Block),
                "minecraft:dark_prismarine" => Blocks::DarkPrismarine(ShapedBlock::Block),
                "minecraft:sea_lantern" => Blocks::SeaLantern,
                "minecraft:warped_wart_block" => Blocks::WarpedWartBlock,
                "minecraft:nether_wart_block" => Blocks::NetherWartBlock,
                "minecraft:red_nether_bricks" => Blocks::RedNetherBrick(ShapedBlock::Block),
                "minecraft:blackstone" => Blocks::Blackstone(ShapedBlock::Block),
                "minecraft:polished_blackstone" => Blocks::PolishedBlackstone(ShapedBlock::Block),
                "minecraft:chiseled_polished_blackstone" => Blocks::ChiseledPolishedBlackstone,
                "minecraft:polished_blackstone_brick" => Blocks::PolishedBlackstoneBrick(ShapedBlock::Block),
                "minecraft:gilded_blackstone" => Blocks::GildedBlackstone,
                "minecraft:crying_obsidian" => Blocks::CryingObsidian,
                "minecraft:dried_kelp_block" => Blocks::DriedKelpBlock,
                "minecraft:cracked_polished_blackstone_bricks" => Blocks::CrackedPolishedBlackstoneBrick,
                "minecraft:infested_cobblestone" => Blocks::InfestedCobblestone,
                "minecraft:infested_stone_bricks" => Blocks::InfestedStoneBricks,
                "minecraft:infested_cracked_stone_bricks" => Blocks::InfestedCrackedStoneBricks,
                "minecraft:infested_mossy_stone_bricks" => Blocks::InfestedMossyStoneBricks,
                "minecraft:infested_chiseled_stone_bricks" => Blocks::InfestedChiseledStoneBricks,
                "minecraft:enchanting_table" => Blocks::EnchantingTable,
                "minecraft:crafting_table" => Blocks::CraftingTable,
                "minecraft:cartography_table" => Blocks::CartographyTable,
                "minecraft:fletching_table" => Blocks::FletchingTable,
                "minecraft:smithing_table" => Blocks::SmithingTable,
                "minecraft:bamboo_sapling" => Blocks::BambooSapling,
                "minecraft:shroomlight" => Blocks::Shroomlight,
                "minecraft:lodestone" => Blocks::Lodestone,
                "minecraft:honeycomb_block" => Blocks::HoneycombBlock,
                "minecraft:lily_pad" => Blocks::LilyPad,
                "minecraft:powder_snow" => Blocks::PowderSnow,
                "minecraft:beacon" => Blocks::Beacon,
                "minecraft:kelp_plant" => Blocks::KelpPlant,
                "minecraft:dragon_egg" => Blocks::DragonEgg,

                "minecraft:glass" => Blocks::Glass(DyeColorClear::Clear),
                "minecraft:white_stained_glass" => Blocks::Glass(DyeColorClear::White),
                "minecraft:red_stained_glass" => Blocks::Glass(DyeColorClear::Red),
                "minecraft:orange_stained_glass" => Blocks::Glass(DyeColorClear::Orange),
                "minecraft:yellow_stained_glass" => Blocks::Glass(DyeColorClear::Yellow),
                "minecraft:green_stained_glass" => Blocks::Glass(DyeColorClear::Green),
                "minecraft:lime_stained_glass" => Blocks::Glass(DyeColorClear::Lime),
                "minecraft:blue_stained_glass" => Blocks::Glass(DyeColorClear::Blue),
                "minecraft:light_blue_stained_glass" => Blocks::Glass(DyeColorClear::LightBlue),
                "minecraft:cyan_stained_glass" => Blocks::Glass(DyeColorClear::Cyan),
                "minecraft:magenta_stained_glass" => Blocks::Glass(DyeColorClear::Magenta),
                "minecraft:pink_stained_glass" => Blocks::Glass(DyeColorClear::Pink),
                "minecraft:purple_stained_glass" => Blocks::Glass(DyeColorClear::Purple),
                "minecraft:brown_stained_glass" => Blocks::Glass(DyeColorClear::Brown),
                "minecraft:gray_stained_glass" => Blocks::Glass(DyeColorClear::Gray),
                "minecraft:light_gray_stained_glass" => Blocks::Glass(DyeColorClear::LightGray),
                "minecraft:black_stained_glass" => Blocks::Glass(DyeColorClear::Black),
                "minecraft:tinted_glass" => Blocks::TintedGlass,

                "minecraft:terracotta" => Blocks::Terracotta(DyeColorClear::Clear),
                "minecraft:white_terracotta" => Blocks::Terracotta(DyeColorClear::White),
                "minecraft:red_terracotta" => Blocks::Terracotta(DyeColorClear::Red),
                "minecraft:orange_terracotta" => Blocks::Terracotta(DyeColorClear::Orange),
                "minecraft:yellow_terracotta" => Blocks::Terracotta(DyeColorClear::Yellow),
                "minecraft:green_terracotta" => Blocks::Terracotta(DyeColorClear::Green),
                "minecraft:lime_terracotta" => Blocks::Terracotta(DyeColorClear::Lime),
                "minecraft:blue_terracotta" => Blocks::Terracotta(DyeColorClear::Blue),
                "minecraft:light_blue_terracotta" => Blocks::Terracotta(DyeColorClear::LightBlue),
                "minecraft:cyan_terracotta" => Blocks::Terracotta(DyeColorClear::Cyan),
                "minecraft:magenta_terracotta" => Blocks::Terracotta(DyeColorClear::Magenta),
                "minecraft:pink_terracotta" => Blocks::Terracotta(DyeColorClear::Pink),
                "minecraft:purple_terracotta" => Blocks::Terracotta(DyeColorClear::Purple),
                "minecraft:brown_terracotta" => Blocks::Terracotta(DyeColorClear::Brown),
                "minecraft:gray_terracotta" => Blocks::Terracotta(DyeColorClear::Gray),
                "minecraft:light_gray_terracotta" => Blocks::Terracotta(DyeColorClear::LightGray),
                "minecraft:black_terracotta" => Blocks::Terracotta(DyeColorClear::Black),

                "minecraft:white_wool" => Blocks::Wool(DyeColor::White),
                "minecraft:red_wool" => Blocks::Wool(DyeColor::Red),
                "minecraft:orange_wool" => Blocks::Wool(DyeColor::Orange),
                "minecraft:yellow_wool" => Blocks::Wool(DyeColor::Yellow),
                "minecraft:green_wool" => Blocks::Wool(DyeColor::Green),
                "minecraft:lime_wool" => Blocks::Wool(DyeColor::Lime),
                "minecraft:blue_wool" => Blocks::Wool(DyeColor::Blue),
                "minecraft:light_blue_wool" => Blocks::Wool(DyeColor::LightBlue),
                "minecraft:cyan_wool" => Blocks::Wool(DyeColor::Cyan),
                "minecraft:magenta_wool" => Blocks::Wool(DyeColor::Magenta),
                "minecraft:pink_wool" => Blocks::Wool(DyeColor::Pink),
                "minecraft:purple_wool" => Blocks::Wool(DyeColor::Purple),
                "minecraft:brown_wool" => Blocks::Wool(DyeColor::Brown),
                "minecraft:gray_wool" => Blocks::Wool(DyeColor::Gray),
                "minecraft:light_gray_wool" => Blocks::Wool(DyeColor::LightGray),
                "minecraft:black_wool" => Blocks::Wool(DyeColor::Black),

                "minecraft:white_carpet" => Blocks::Carpet(DyeColor::White),
                "minecraft:red_carpet" => Blocks::Carpet(DyeColor::Red),
                "minecraft:orange_carpet" => Blocks::Carpet(DyeColor::Orange),
                "minecraft:yellow_carpet" => Blocks::Carpet(DyeColor::Yellow),
                "minecraft:green_carpet" => Blocks::Carpet(DyeColor::Green),
                "minecraft:lime_carpet" => Blocks::Carpet(DyeColor::Lime),
                "minecraft:blue_carpet" => Blocks::Carpet(DyeColor::Blue),
                "minecraft:light_blue_carpet" => Blocks::Carpet(DyeColor::LightBlue),
                "minecraft:cyan_carpet" => Blocks::Carpet(DyeColor::Cyan),
                "minecraft:magenta_carpet" => Blocks::Carpet(DyeColor::Magenta),
                "minecraft:pink_carpet" => Blocks::Carpet(DyeColor::Pink),
                "minecraft:purple_carpet" => Blocks::Carpet(DyeColor::Purple),
                "minecraft:brown_carpet" => Blocks::Carpet(DyeColor::Brown),
                "minecraft:gray_carpet" => Blocks::Carpet(DyeColor::Gray),
                "minecraft:light_gray_carpet" => Blocks::Carpet(DyeColor::LightGray),
                "minecraft:black_carpet" => Blocks::Carpet(DyeColor::Black),

                "minecraft:white_concrete" => Blocks::Concrete(DyeColor::White),
                "minecraft:red_concrete" => Blocks::Concrete(DyeColor::Red),
                "minecraft:orange_concrete" => Blocks::Concrete(DyeColor::Orange),
                "minecraft:yellow_concrete" => Blocks::Concrete(DyeColor::Yellow),
                "minecraft:green_concrete" => Blocks::Concrete(DyeColor::Green),
                "minecraft:lime_concrete" => Blocks::Concrete(DyeColor::Lime),
                "minecraft:blue_concrete" => Blocks::Concrete(DyeColor::Blue),
                "minecraft:light_blue_concrete" => Blocks::Concrete(DyeColor::LightBlue),
                "minecraft:cyan_concrete" => Blocks::Concrete(DyeColor::Cyan),
                "minecraft:magenta_concrete" => Blocks::Concrete(DyeColor::Magenta),
                "minecraft:pink_concrete" => Blocks::Concrete(DyeColor::Pink),
                "minecraft:purple_concrete" => Blocks::Concrete(DyeColor::Purple),
                "minecraft:brown_concrete" => Blocks::Concrete(DyeColor::Brown),
                "minecraft:gray_concrete" => Blocks::Concrete(DyeColor::Gray),
                "minecraft:light_gray_concrete" => Blocks::Concrete(DyeColor::LightGray),
                "minecraft:black_concrete" => Blocks::Concrete(DyeColor::Black),

                "minecraft:white_concrete_powder" => Blocks::ConcretePowder(DyeColor::White),
                "minecraft:red_concrete_powder" => Blocks::ConcretePowder(DyeColor::Red),
                "minecraft:orange_concrete_powder" => Blocks::ConcretePowder(DyeColor::Orange),
                "minecraft:yellow_concrete_powder" => Blocks::ConcretePowder(DyeColor::Yellow),
                "minecraft:green_concrete_powder" => Blocks::ConcretePowder(DyeColor::Green),
                "minecraft:lime_concrete_powder" => Blocks::ConcretePowder(DyeColor::Lime),
                "minecraft:blue_concrete_powder" => Blocks::ConcretePowder(DyeColor::Blue),
                "minecraft:light_blue_concrete_powder" => Blocks::ConcretePowder(DyeColor::LightBlue),
                "minecraft:cyan_concrete_powder" => Blocks::ConcretePowder(DyeColor::Cyan),
                "minecraft:magenta_concrete_powder" => Blocks::ConcretePowder(DyeColor::Magenta),
                "minecraft:pink_concrete_powder" => Blocks::ConcretePowder(DyeColor::Pink),
                "minecraft:purple_concrete_powder" => Blocks::ConcretePowder(DyeColor::Purple),
                "minecraft:brown_concrete_powder" => Blocks::ConcretePowder(DyeColor::Brown),
                "minecraft:gray_concrete_powder" => Blocks::ConcretePowder(DyeColor::Gray),
                "minecraft:light_gray_concrete_powder" => Blocks::ConcretePowder(DyeColor::LightGray),
                "minecraft:black_concrete_powder" => Blocks::ConcretePowder(DyeColor::Black),

                "minecraft:brain_coral_block" => Blocks::Coral(CoralType::Brain, Dead::Alive, CoralShape::Block),
                "minecraft:bubble_coral_block" => Blocks::Coral(CoralType::Bubble, Dead::Alive, CoralShape::Block),
                "minecraft:fire_coral_block" => Blocks::Coral(CoralType::Fire, Dead::Alive, CoralShape::Block),
                "minecraft:horn_coral_block" => Blocks::Coral(CoralType::Horn, Dead::Alive, CoralShape::Block),
                "minecraft:tube_coral_block" => Blocks::Coral(CoralType::Tube, Dead::Alive, CoralShape::Block),
                "minecraft:dead_brain_coral_block" => Blocks::Coral(CoralType::Brain, Dead::Dead, CoralShape::Block),
                "minecraft:dead_bubble_coral_block" => Blocks::Coral(CoralType::Bubble, Dead::Dead, CoralShape::Block),
                "minecraft:dead_fire_coral_block" => Blocks::Coral(CoralType::Fire, Dead::Dead, CoralShape::Block),
                "minecraft:dead_horn_coral_block" => Blocks::Coral(CoralType::Horn, Dead::Dead, CoralShape::Block),
                "minecraft:dead_tube_coral_block" => Blocks::Coral(CoralType::Tube, Dead::Dead, CoralShape::Block),

                _ => panic!("{}", name)
            }
        }
        Some(nbt) => {
            match name {
                "minecraft:grass_block" => Blocks::GrassBlock(GrassType::Grass, get_snowy(nbt)),
                "minecraft:podzol" => Blocks::GrassBlock(GrassType::Podzol, get_snowy(nbt)),
                "minecraft:mycelium" => Blocks::GrassBlock(GrassType::Mycelium, get_snowy(nbt)),
                "minecraft:deepslate" => Blocks::Deepslate(get_axis(nbt)),
                "minecraft:basalt" => Blocks::Basalt(get_axis(nbt)),
                "minecraft:polished_basalt" => Blocks::PolishedBasalt(get_axis(nbt)),
                "minecraft:bone_block" => Blocks::BoneBlock(get_axis(nbt)),
                "minecraft:quartz_pillar" => Blocks::QuartzPillar(get_axis(nbt)),
                "minecraft:infested_deepslate" => {
                    Blocks::InfestedDeepslate(get_axis(nbt))
                }
                "minecraft:deepslate_redstone_ore" => {
                    Blocks::DeepslateOre(OreType::Redstone(get_lit(nbt)))
                }
                "minecraft:redstone_ore" => Blocks::Ore(OreType::Redstone(get_lit(nbt))),
                "minecraft:water" => Blocks::Water(get_level15(nbt)),
                "minecraft:lava" => Blocks::Lava(get_level15(nbt)),
                "minecraft:glow_lichen" => {
                    Blocks::GlowLichen(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up),
                    get_side(nbt, Direction6::Down),
                    get_waterlogged(nbt))
                }
                "minecraft:vine" => {
                    Blocks::Vine(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up))
                }
                "minecraft:sculk_vein" => {
                    Blocks::SculkVein(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up),
                    get_side(nbt, Direction6::Down),
                    get_waterlogged(nbt))
                }
                "minecraft:sculk_catalyst" => Blocks::SculkCatalist(get_bloom(nbt)),
                "minecraft:sculk_sensor" => {
                    Blocks::SculkSensor(get_power(nbt), get_waterlogged(nbt), get_sculk_sensor_phase(nbt))
                }
                "minecraft:sculk_shrieker" => {
                    Blocks::SculkShrieker(get_can_summon(nbt), get_waterlogged(nbt), get_shrieking(nbt))
                }
                "minecraft:chest" => {
                    Blocks::Chest(Trapped::False, get_chest_half(nbt), get_facing4(nbt), get_waterlogged(nbt))
                }
                "minecraft:trapped_chest" => {
                    Blocks::Chest(Trapped::True, get_chest_half(nbt), get_facing4(nbt), get_waterlogged(nbt))
                }
                "minecraft:ender_chest" => Blocks::EnderChest(get_facing4(nbt), get_waterlogged(nbt)),
                "minecraft:end_rod" => Blocks::EndRod(get_facing6(nbt)),
                "minecraft:pointed_dripstone" => {
                    Blocks::PointedDripstone(get_vertical_direction(nbt), get_thickness(nbt), get_waterlogged(nbt))
                }
                "minecraft:purpur_pillar" => Blocks::PurpurPillar(get_axis(nbt)),
                "minecraft:ochre_froglight" => Blocks::OchreFroglight(get_axis(nbt)),
                "minecraft:verdant_froglight" => Blocks::VerdantFroglight(get_axis(nbt)),
                "minecraft:pearlescent_froglight" => Blocks::PearlescentFroglight(get_axis(nbt)),

                "minecraft:stone_slab" => Blocks::Stone(get_slab(nbt)),
                "minecraft:cobblestone_wall" => Blocks::Cobblestone(get_wall(nbt)),
                "minecraft:stone_stairs" => Blocks::Stone(get_stair(nbt)),
                "minecraft:andesite_slab" => Blocks::Andesite(get_slab(nbt)),
                "minecraft:andesite_stairs" => Blocks::Andesite(get_stair(nbt)),
                "minecraft:andesite_wall" => Blocks::Andesite(get_wall(nbt)),
                "minecraft:diorite_slab" => Blocks::Diorite(get_slab(nbt)),
                "minecraft:diorite_stairs" => Blocks::Diorite(get_stair(nbt)),
                "minecraft:diorite_wall" => Blocks::Diorite(get_wall(nbt)),
                "minecraft:granite_slab" => Blocks::Granite(get_slab(nbt)),
                "minecraft:granite_stairs" => Blocks::Granite(get_stair(nbt)),
                "minecraft:granite_wall" => Blocks::Granite(get_wall(nbt)),
                "minecraft:polished_andesite_slab" => Blocks::PolishedAndesite(get_slab(nbt)),
                "minecraft:polished_andesite_stairs" => Blocks::PolishedAndesite(get_stair(nbt)),
                "minecraft:polished_diorite_slab" => Blocks::PolishedDiorite(get_slab(nbt)),
                "minecraft:polished_diorite_stairs" => Blocks::PolishedDiorite(get_stair(nbt)),
                "minecraft:polished_granite_slab" => Blocks::PolishedGranite(get_slab(nbt)),
                "minecraft:polished_granite_stairs" => Blocks::PolishedGranite(get_stair(nbt)),
                "minecraft:cobblestone_slab" => Blocks::Cobblestone(get_slab(nbt)),
                "minecraft:cobblestone_stairs" => Blocks::Cobblestone(get_stair(nbt)),
                "minecraft:mossy_cobblestone_slab" => Blocks::MossyCobblestone(get_slab(nbt)),
                "minecraft:mossy_cobblestone_stairs" => Blocks::MossyCobblestone(get_stair(nbt)),
                "minecraft:mossy_cobblestone_wall" => Blocks::MossyCobblestone(get_wall(nbt)),
                "minecraft:stone_brick_slab" => Blocks::StoneBrick(get_slab(nbt)),
                "minecraft:stone_brick_stairs" => Blocks::StoneBrick(get_stair(nbt)),
                "minecraft:stone_brick_wall" => Blocks::StoneBrick(get_wall(nbt)),
                "minecraft:mossy_stone_brick_slab" => Blocks::MossyStoneBrick(get_slab(nbt)),
                "minecraft:mossy_stone_brick_stairs" => Blocks::MossyStoneBrick(get_stair(nbt)),
                "minecraft:mossy_stone_brick_wall" => Blocks::MossyStoneBrick(get_wall(nbt)),
                "minecraft:mud_brick_slab" => Blocks::MudBrick(get_slab(nbt)),
                "minecraft:mud_brick_stairs" => Blocks::MudBrick(get_stair(nbt)),
                "minecraft:mud_brick_wall" => Blocks::MudBrick(get_wall(nbt)),
                "minecraft:nether_brick_slab" => Blocks::NetherBrick(get_slab(nbt)),
                "minecraft:nether_brick_stairs" => Blocks::NetherBrick(get_stair(nbt)),
                "minecraft:nether_brick_fence" => Blocks::NetherBrick(get_fence(nbt)),
                "minecraft:nether_brick_wall" => Blocks::NetherBrick(get_wall(nbt)),
                "minecraft:red_nether_brick_slab" => Blocks::RedNetherBrick(get_slab(nbt)),
                "minecraft:red_nether_brick_stairs" => Blocks::RedNetherBrick(get_stair(nbt)),
                "minecraft:red_nether_brick_wall" => Blocks::RedNetherBrick(get_wall(nbt)),
                "minecraft:brick_slab" => Blocks::Brick(get_slab(nbt)),
                "minecraft:brick_stairs" => Blocks::Brick(get_stair(nbt)),
                "minecraft:brick_wall" => Blocks::Brick(get_wall(nbt)),
                "minecraft:sandstone_slab" => Blocks::Sandstone(Red::Normal, get_slab(nbt)),
                "minecraft:sandstone_stairs" => Blocks::Sandstone(Red::Normal, get_stair(nbt)),
                "minecraft:sandstone_wall" => Blocks::Sandstone(Red::Normal, get_wall(nbt)),
                "minecraft:red_sandstone_slab" => Blocks::Sandstone(Red::Red, get_slab(nbt)),
                "minecraft:red_sandstone_stairs" => Blocks::Sandstone(Red::Red, get_stair(nbt)),
                "minecraft:red_sandstone_wall" => Blocks::Sandstone(Red::Red, get_wall(nbt)),
                "minecraft:smooth_sandstone_slab" => Blocks::SmoothSandstone(Red::Normal, get_slab(nbt)),
                "minecraft:smooth_sandstone_stairs" => Blocks::SmoothSandstone(Red::Normal, get_stair(nbt)),
                "minecraft:smooth_red_sandstone_slab" => Blocks::SmoothSandstone(Red::Red, get_slab(nbt)),
                "minecraft:smooth_red_sandstone_stairs" => Blocks::SmoothSandstone(Red::Red, get_stair(nbt)),
                "minecraft:cut_sandstone_slab" => Blocks::CutSandstone(Red::Normal, get_slab(nbt)),
                "minecraft:cut_red_sandstone_slab" => Blocks::CutSandstone(Red::Red, get_slab(nbt)),
                //edgecase I cant be bothered with, just make it an oak slab
                "minecraft:petrified_oak_slab" => Blocks::Planks(WoodType::Oak, get_slab(nbt)),
                "minecraft:quartz_slab" => Blocks::Quartz(get_slab(nbt)),
                "minecraft:quartz_stairs" => Blocks::Quartz(get_stair(nbt)),
                "minecraft:smooth_quartz_slab" => Blocks::SmoothQuartz(get_slab(nbt)),
                "minecraft:smooth_quartz_stairs" => Blocks::SmoothQuartz(get_stair(nbt)),
                "minecraft:purpur_slab" => Blocks::Purpur(get_slab(nbt)),
                "minecraft:purpur_stairs" => Blocks::Purpur(get_stair(nbt)),
                "minecraft:prismarine_slab" => Blocks::Prismarine(get_slab(nbt)),
                "minecraft:prismarine_stairs" => Blocks::Prismarine(get_stair(nbt)),
                "minecraft:prismarine_wall" => Blocks::Prismarine(get_wall(nbt)),
                "minecraft:dark_prismarine_slab" => Blocks::DarkPrismarine(get_slab(nbt)),
                "minecraft:dark_prismarine_stairs" => Blocks::DarkPrismarine(get_stair(nbt)),
                "minecraft:prismarine_brick_slab" => Blocks::PrismarineBrick(get_slab(nbt)),
                "minecraft:prismarine_brick_stairs" => Blocks::PrismarineBrick(get_stair(nbt)),
                "minecraft:end_stone_brick_slab" => Blocks::EndStoneBrick(get_slab(nbt)),
                "minecraft:end_stone_brick_stairs" => Blocks::EndStoneBrick(get_stair(nbt)),
                "minecraft:end_stone_brick_wall" => Blocks::EndStoneBrick(get_wall(nbt)),
                "minecraft:deepslate_tile_slab" => Blocks::DeepslateTile(get_slab(nbt)),
                "minecraft:deepslate_tile_stairs" => Blocks::DeepslateTile(get_stair(nbt)),
                "minecraft:deepslate_tile_wall" => Blocks::DeepslateTile(get_wall(nbt)),
                "minecraft:deepslate_brick_slab" => Blocks::DeepslateBricks(get_slab(nbt)),
                "minecraft:deepslate_brick_stairs" => Blocks::DeepslateBricks(get_stair(nbt)),
                "minecraft:deepslate_brick_wall" => Blocks::DeepslateBricks(get_wall(nbt)),
                "minecraft:cobbled_deepslate_slab" => Blocks::CobbledDeepslate(get_slab(nbt)),
                "minecraft:cobbled_deepslate_stairs" => Blocks::CobbledDeepslate(get_stair(nbt)),
                "minecraft:cobbled_deepslate_wall" => Blocks::CobbledDeepslate(get_wall(nbt)),
                "minecraft:polished_deepslate_slab" => Blocks::PolishedDeepslate(get_slab(nbt)),
                "minecraft:polished_deepslate_stairs" => Blocks::PolishedDeepslate(get_stair(nbt)),
                "minecraft:polished_deepslate_wall" => Blocks::PolishedDeepslate(get_wall(nbt)),
                "minecraft:blackstone_slab" => Blocks::Blackstone(get_slab(nbt)),
                "minecraft:blackstone_stairs" => Blocks::Blackstone(get_stair(nbt)),
                "minecraft:blackstone_wall" => Blocks::Blackstone(get_wall(nbt)),
                "minecraft:polished_blackstone_slab" => Blocks::PolishedBlackstone(get_slab(nbt)),
                "minecraft:polished_blackstone_stairs" => Blocks::PolishedBlackstone(get_stair(nbt)),
                "minecraft:polished_blackstone_wall" => Blocks::PolishedBlackstone(get_wall(nbt)),
                "minecraft:polished_blackstone_brick_slab" => Blocks::PolishedBlackstoneBrick(get_slab(nbt)),
                "minecraft:polished_blackstone_brick_stairs" => Blocks::PolishedBlackstoneBrick(get_stair(nbt)),
                "minecraft:polished_blackstone_brick_wall" => Blocks::PolishedBlackstoneBrick(get_wall(nbt)),

                /////// wood start

                "minecraft:oak_log" => Blocks::Log(WoodType::Oak, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_oak_log" => Blocks::Log(WoodType::Oak, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:oak_wood" => Blocks::Log(WoodType::Oak, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_oak_wood" => Blocks::Log(WoodType::Oak, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:oak_stairs" => Blocks::Planks(WoodType::Oak, get_stair(nbt)),
                "minecraft:oak_slab" => Blocks::Planks(WoodType::Oak, get_slab(nbt)),
                "minecraft:oak_fence" => {
                    Blocks::Planks(WoodType::Oak, get_fence(nbt))
                }
                "minecraft:oak_leaves" => {
                    Blocks::Leaves(LeafType::Oak, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:oak_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Oak, get_powered(nbt))
                }
                "minecraft:oak_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Oak,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:oak_door" => {
                    Blocks::Door(DoorType::Oak,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:oak_fence_gate" => {
                    Blocks::FenceGate(WoodType::Oak, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:oak_sapling" => Blocks::Sapling(SaplingType::Oak, get_stage1(nbt)),
                "minecraft:oak_button" => Blocks::Button(ButtonType::Oak, get_powered(nbt)),
                "minecraft:oak_wall_sign" => Blocks::Sign(WoodType::Oak, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:oak_sign" => Blocks::Sign(WoodType::Oak, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // birch

                "minecraft:birch_log" => Blocks::Log(WoodType::Birch, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_birch_log" => Blocks::Log(WoodType::Birch, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:birch_wood" => Blocks::Log(WoodType::Birch, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_birch_wood" => Blocks::Log(WoodType::Birch, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:birch_stairs" => Blocks::Planks(WoodType::Birch, get_stair(nbt)),
                "minecraft:birch_slab" => Blocks::Planks(WoodType::Birch, get_slab(nbt)),
                "minecraft:birch_fence" => {
                    Blocks::Planks(WoodType::Birch, get_fence(nbt))
                }
                "minecraft:birch_leaves" => {
                    Blocks::Leaves(LeafType::Birch, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:birch_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Birch, get_powered(nbt))
                }
                "minecraft:birch_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Birch,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:birch_door" => {
                    Blocks::Door(DoorType::Birch,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:birch_fence_gate" => {
                    Blocks::FenceGate(WoodType::Birch, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:birch_sapling" => Blocks::Sapling(SaplingType::Birch, get_stage1(nbt)),
                "minecraft:birch_button" => Blocks::Button(ButtonType::Birch, get_powered(nbt)),
                "minecraft:birch_wall_sign" => Blocks::Sign(WoodType::Birch, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:birch_sign" => Blocks::Sign(WoodType::Birch, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // spruce

                "minecraft:spruce_log" => Blocks::Log(WoodType::Spruce, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_spruce_log" => Blocks::Log(WoodType::Spruce, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:spruce_wood" => Blocks::Log(WoodType::Spruce, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_spruce_wood" => Blocks::Log(WoodType::Spruce, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:spruce_stairs" => Blocks::Planks(WoodType::Spruce, get_stair(nbt)),
                "minecraft:spruce_slab" => Blocks::Planks(WoodType::Spruce, get_slab(nbt)),
                "minecraft:spruce_fence" => {
                    Blocks::Planks(WoodType::Spruce, get_fence(nbt))
                }
                "minecraft:spruce_leaves" => {
                    Blocks::Leaves(LeafType::Spruce, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:spruce_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Spruce, get_powered(nbt))
                }
                "minecraft:spruce_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Spruce,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:spruce_door" => {
                    Blocks::Door(DoorType::Spruce,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:spruce_fence_gate" => {
                    Blocks::FenceGate(WoodType::Spruce, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:spruce_sapling" => Blocks::Sapling(SaplingType::Spruce, get_stage1(nbt)),
                "minecraft:spruce_button" => Blocks::Button(ButtonType::Spruce, get_powered(nbt)),
                "minecraft:spruce_wall_sign" => Blocks::Sign(WoodType::Spruce, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:spruce_sign" => Blocks::Sign(WoodType::Spruce, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // jungle

                "minecraft:jungle_log" => Blocks::Log(WoodType::Jungle, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_jungle_log" => Blocks::Log(WoodType::Jungle, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:jungle_wood" => Blocks::Log(WoodType::Jungle, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_jungle_wood" => Blocks::Log(WoodType::Jungle, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:jungle_stairs" => Blocks::Planks(WoodType::Jungle, get_stair(nbt)),
                "minecraft:jungle_slab" => Blocks::Planks(WoodType::Jungle, get_slab(nbt)),
                "minecraft:jungle_fence" => {
                    Blocks::Planks(WoodType::Jungle, get_fence(nbt))
                }
                "minecraft:jungle_leaves" => {
                    Blocks::Leaves(LeafType::Jungle, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:jungle_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Jungle, get_powered(nbt))
                }
                "minecraft:jungle_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Jungle,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:jungle_door" => {
                    Blocks::Door(DoorType::Jungle,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:jungle_fence_gate" => {
                    Blocks::FenceGate(WoodType::Jungle, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:jungle_sapling" => Blocks::Sapling(SaplingType::Jungle, get_stage1(nbt)),
                "minecraft:jungle_button" => Blocks::Button(ButtonType::Jungle, get_powered(nbt)),
                "minecraft:jungle_wall_sign" => Blocks::Sign(WoodType::Jungle, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:jungle_sign" => Blocks::Sign(WoodType::Jungle, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // dark oak

                "minecraft:dark_oak_log" => Blocks::Log(WoodType::DarkOak, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_dark_oak_log" => Blocks::Log(WoodType::DarkOak, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:dark_oak_wood" => Blocks::Log(WoodType::DarkOak, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_dark_oak_wood" => Blocks::Log(WoodType::DarkOak, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:dark_oak_stairs" => Blocks::Planks(WoodType::DarkOak, get_stair(nbt)),
                "minecraft:dark_oak_slab" => Blocks::Planks(WoodType::DarkOak, get_slab(nbt)),
                "minecraft:dark_oak_fence" => {
                    Blocks::Planks(WoodType::DarkOak, get_fence(nbt))
                }
                "minecraft:dark_oak_leaves" => {
                    Blocks::Leaves(LeafType::DarkOak, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:dark_oak_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::DarkOak, get_powered(nbt))
                }
                "minecraft:dark_oak_trapdoor" => {
                    Blocks::Trapdoor(DoorType::DarkOak,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:dark_oak_door" => {
                    Blocks::Door(DoorType::DarkOak,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:dark_oak_fence_gate" => {
                    Blocks::FenceGate(WoodType::DarkOak, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:dark_oak_sapling" => Blocks::Sapling(SaplingType::DarkOak, get_stage1(nbt)),
                "minecraft:dark_oak_button" => Blocks::Button(ButtonType::DarkOak, get_powered(nbt)),
                "minecraft:dark_oak_wall_sign" => Blocks::Sign(WoodType::DarkOak, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:dark_oak_sign" => Blocks::Sign(WoodType::DarkOak, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // acacia

                "minecraft:acacia_log" => Blocks::Log(WoodType::Acacia, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_acacia_log" => Blocks::Log(WoodType::Acacia, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:acacia_wood" => Blocks::Log(WoodType::Acacia, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_acacia_wood" => Blocks::Log(WoodType::Acacia, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:acacia_stairs" => Blocks::Planks(WoodType::Acacia, get_stair(nbt)),
                "minecraft:acacia_slab" => Blocks::Planks(WoodType::Acacia, get_slab(nbt)),
                "minecraft:acacia_fence" => {
                    Blocks::Planks(WoodType::Acacia, get_fence(nbt))
                }
                "minecraft:acacia_leaves" => {
                    Blocks::Leaves(LeafType::Acacia, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:acacia_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Acacia, get_powered(nbt))
                }
                "minecraft:acacia_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Acacia,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:acacia_door" => {
                    Blocks::Door(DoorType::Acacia,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:acacia_fence_gate" => {
                    Blocks::FenceGate(WoodType::Acacia, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:acacia_sapling" => Blocks::Sapling(SaplingType::Acacia, get_stage1(nbt)),
                "minecraft:acacia_button" => Blocks::Button(ButtonType::Acacia, get_powered(nbt)),
                "minecraft:acacia_wall_sign" => Blocks::Sign(WoodType::Acacia, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:acacia_sign" => Blocks::Sign(WoodType::Acacia, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // mangrove

                "minecraft:mangrove_log" => Blocks::Log(WoodType::Mangrove, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_mangrove_log" => Blocks::Log(WoodType::Mangrove, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:mangrove_wood" => Blocks::Log(WoodType::Mangrove, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_mangrove_wood" => Blocks::Log(WoodType::Mangrove, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:mangrove_stairs" => Blocks::Planks(WoodType::Mangrove, get_stair(nbt)),
                "minecraft:mangrove_slab" => Blocks::Planks(WoodType::Mangrove, get_slab(nbt)),
                "minecraft:mangrove_fence" => {
                    Blocks::Planks(WoodType::Mangrove, get_fence(nbt))
                }
                "minecraft:mangrove_leaves" => {
                    Blocks::Leaves(LeafType::Mangrove, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:mangrove_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Mangrove, get_powered(nbt))
                }
                "minecraft:mangrove_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Mangrove,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:mangrove_door" => {
                    Blocks::Door(DoorType::Mangrove,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:mangrove_fence_gate" => {
                    Blocks::FenceGate(WoodType::Mangrove, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:mangrove_roots" => Blocks::MangroveRoots(get_waterlogged(nbt)),
                "minecraft:muddy_mangrove_roots" => Blocks::MuddyMangroveRoots(get_axis(nbt)),

                "minecraft:mangrove_propagule" => {//weird
                    Blocks::MangrovePropagule(get_age3(nbt), get_hanging(nbt), get_stage1(nbt), get_waterlogged(nbt))
                }
                "minecraft:mangrove_button" => Blocks::Button(ButtonType::Mangrove, get_powered(nbt)),
                "minecraft:mangrove_wall_sign" => Blocks::Sign(WoodType::Mangrove, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:mangrove_sign" => Blocks::Sign(WoodType::Mangrove, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // crimson

                "minecraft:crimson_stem" => Blocks::Log(WoodType::Crimson, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_crimson_stem" => Blocks::Log(WoodType::Crimson, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:crimson_hyphae" => Blocks::Log(WoodType::Crimson, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_crimson_hyphae" => Blocks::Log(WoodType::Crimson, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:crimson_stairs" => Blocks::Planks(WoodType::Crimson, get_stair(nbt)),
                "minecraft:crimson_slab" => Blocks::Planks(WoodType::Crimson, get_slab(nbt)),
                "minecraft:crimson_fence" => {
                    Blocks::Planks(WoodType::Crimson, get_fence(nbt))
                }
                "minecraft:crimson_leaves" => {
                    Blocks::Leaves(LeafType::Crimson, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:crimson_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Crimson, get_powered(nbt))
                }
                "minecraft:crimson_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Crimson,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:crimson_door" => {
                    Blocks::Door(DoorType::Crimson,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:crimson_fence_gate" => {
                    Blocks::FenceGate(WoodType::Crimson, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:crimson_sapling" => Blocks::Sapling(SaplingType::Crimson, get_stage1(nbt)),
                "minecraft:crimson_button" => Blocks::Button(ButtonType::Crimson, get_powered(nbt)),
                "minecraft:crimson_wall_sign" => Blocks::Sign(WoodType::Crimson, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:crimson_sign" => Blocks::Sign(WoodType::Crimson, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                // warped

                "minecraft:warped_stem" => Blocks::Log(WoodType::Warped, LogType::Log, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_warped_stem" => Blocks::Log(WoodType::Warped, LogType::Log, Stripped::Stripped, get_axis(nbt)),
                "minecraft:warped_hyphae" => Blocks::Log(WoodType::Warped, LogType::Wood, Stripped::Unstripped, get_axis(nbt)),
                "minecraft:stripped_warped_hyphae" => Blocks::Log(WoodType::Warped, LogType::Wood, Stripped::Stripped, get_axis(nbt)),
                "minecraft:warped_stairs" => Blocks::Planks(WoodType::Warped, get_stair(nbt)),
                "minecraft:warped_slab" => Blocks::Planks(WoodType::Warped, get_slab(nbt)),
                "minecraft:warped_fence" => {
                    Blocks::Planks(WoodType::Warped, get_fence(nbt))
                }
                "minecraft:warped_leaves" => {
                    Blocks::Leaves(LeafType::Warped, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:warped_pressure_plate" => {
                    Blocks::PressurePlate(PressurePlateType::Warped, get_powered(nbt))
                }
                "minecraft:warped_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Warped,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:warped_door" => {
                    Blocks::Door(DoorType::Warped,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:warped_fence_gate" => {
                    Blocks::FenceGate(WoodType::Warped, get_open(nbt), get_powered(nbt), get_facing4(nbt), get_in_wall(nbt))
                }
                "minecraft:warped_sapling" => Blocks::Sapling(SaplingType::Warped, get_stage1(nbt)),
                "minecraft:warped_button" => Blocks::Button(ButtonType::Warped, get_powered(nbt)),
                "minecraft:warped_wall_sign" => Blocks::Sign(WoodType::Warped, Sign::WallSign(get_facing4(nbt)), get_waterlogged(nbt)),
                "minecraft:warped_sign" => Blocks::Sign(WoodType::Warped, Sign::Sign(get_rotation(nbt)), get_waterlogged(nbt)),

                ////// azalea

                "minecraft:flowering_azalea_leaves" => {
                    Blocks::Leaves(LeafType::FloweringAzalea, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }
                "minecraft:azalea_leaves" => {
                    Blocks::Leaves(LeafType::Azalea, get_distance(nbt), get_persistent(nbt), get_waterlogged(nbt))
                }

                /////// wood end

                "minecraft:iron_door" => {
                    Blocks::Door(DoorType::Iron,
                        get_open(nbt),
                        get_powered(nbt),
                        get_hinge_side(nbt),
                        get_half(nbt),
                        get_facing4(nbt))
                }
                "minecraft:iron_trapdoor" => {
                    Blocks::Trapdoor(DoorType::Iron,
                        get_waterlogged(nbt),
                        get_powered(nbt),
                        get_stair_up_down(nbt),
                        get_open(nbt),
                        get_facing4(nbt))
                }
                "minecraft:hanging_roots" => Blocks::HangingRoots(get_waterlogged(nbt)),
                "minecraft:iron_bars" => {
                    Blocks::IronBars(
                        get_side(nbt, Direction6::North),
                        get_side(nbt, Direction6::East),
                        get_side(nbt, Direction6::South),
                        get_side(nbt, Direction6::West),
                        get_waterlogged(nbt))
                }
                "minecraft:small_amethyst_bud" => Blocks::AmethystCluster(Level3::Zero, get_facing6(nbt), get_waterlogged(nbt)),
                "minecraft:medium_amethyst_bud" => Blocks::AmethystCluster(Level3::One, get_facing6(nbt), get_waterlogged(nbt)),
                "minecraft:large_amethyst_bud" => Blocks::AmethystCluster(Level3::Two, get_facing6(nbt), get_waterlogged(nbt)),
                "minecraft:amethyst_cluster" => Blocks::AmethystCluster(Level3::Three, get_facing6(nbt), get_waterlogged(nbt)),
                "minecraft:hay_block" => Blocks::HayBlock(get_axis(nbt)),
                "minecraft:wall_torch" => Blocks::Torch(Soul::Normal, get_facing4(nbt).into()),
                "minecraft:soul_wall_torch" => Blocks::Torch(Soul::Soul, get_facing4(nbt).into()),
                "minecraft:furnace" => Blocks::Furnace(get_facing4(nbt), get_lit(nbt)),
                "minecraft:blast_furnace" => Blocks::BlastFurnace(get_facing4(nbt), get_lit(nbt)),
                "minecraft:smoker" => Blocks::Smoker(get_facing4(nbt), get_lit(nbt)),
                "minecraft:smooth_stone_slab" => Blocks::SmoothStone(get_slab(nbt)),
                "minecraft:water_cauldron" => Blocks::WaterCauldron(get_level3(nbt)),
                "minecraft:grindstone" => Blocks::Grindstone(get_wall_face(nbt), get_facing4(nbt)),

                "minecraft:anvil" => Blocks::Anvil(AnvilDamage::Clean, get_facing4(nbt)),
                "minecraft:chipped_anvil" => Blocks::Anvil(AnvilDamage::Chipped, get_facing4(nbt)),
                "minecraft:damaged_anvil" => Blocks::Anvil(AnvilDamage::Damaged, get_facing4(nbt)),

                "minecraft:cactus" => Blocks::Flower(Flower::Cactus), //ignoring age
                "minecraft:sugar_cane" => Blocks::SugarCane(get_age15(nbt)), //ignoring age
                "minecraft:ladder" => Blocks::Ladder(get_waterlogged(nbt), get_facing4(nbt)),
                "minecraft:bell" => Blocks::Bell(get_powered(nbt), get_facing4(nbt), get_attachment(nbt)),

                "minecraft:tall_grass" => Blocks::TallFlower(TallFlower::TallGrass, get_half(nbt)),
                "minecraft:large_fern" => Blocks::TallFlower(TallFlower::LargeFern, get_half(nbt)),
                "minecraft:tall_seagrass" => Blocks::TallFlower(TallFlower::TallSeagrass, get_half(nbt)),
                "minecraft:peony" => Blocks::TallFlower(TallFlower::Peony, get_half(nbt)),
                "minecraft:lilac" => Blocks::TallFlower(TallFlower::Lilac, get_half(nbt)),
                "minecraft:rose_bush" => Blocks::TallFlower(TallFlower::Rose, get_half(nbt)),
                "minecraft:sunflower" => Blocks::TallFlower(TallFlower::Sunflower, get_half(nbt)),

                "minecraft:bubble_column" => Blocks::BubbleColumn(get_drag(nbt)),
                "minecraft:cave_vines" => Blocks::CaveVines(get_berries(nbt), get_age24(nbt)),
                "minecraft:cave_vines_plant" => Blocks::CaveVinesPlant(get_berries(nbt), get_age24(nbt)),
                "minecraft:weeping_vines" => Blocks::WeepingVines(get_note_age25(nbt)),
                "minecraft:twisting_vines" => Blocks::TwistingVines(get_note_age25(nbt)),
                "minecraft:bee_nest" => Blocks::BeeNest(get_honey_level(nbt), get_facing4(nbt)),
                "minecraft:beehive" => Blocks::BeeHive(get_honey_level(nbt), get_facing4(nbt)),
                "minecraft:big_dripleaf" => Blocks::BigDripleaf(get_tilt(nbt), get_facing4(nbt), get_waterlogged(nbt)),
                "minecraft:big_dripleaf_stem" => Blocks::BigDripleafStem(get_facing4(nbt), get_waterlogged(nbt)),
                "minecraft:small_dripleaf" => Blocks::SmallDripleaf(get_half(nbt), get_facing4(nbt), get_waterlogged(nbt)),

                "minecraft:rail" => Blocks::Rail(get_rail_shape(nbt), get_waterlogged(nbt)),
                "minecraft:activator_rail" => Blocks::ActivatorRail(get_straight_rail_shape(nbt), get_waterlogged(nbt)),
                "minecraft:detector_rail" => Blocks::DetectorRail(get_straight_rail_shape(nbt), get_waterlogged(nbt)),
                "minecraft:powered_rail" => Blocks::PoweredRail(get_straight_rail_shape(nbt), get_waterlogged(nbt)),

                "minecraft:redstone_wire" => {
                    Blocks::RedstoneWire(get_redstone_connection(nbt, Direction4::North),
                        get_redstone_connection(nbt, Direction4::East),
                        get_redstone_connection(nbt, Direction4::South),
                        get_redstone_connection(nbt, Direction4::West),
                        get_power(nbt))
                }
                "minecraft:observer" => Blocks::Observer(get_powered(nbt), get_facing6(nbt)),
                "minecraft:tripwire_hook" => Blocks::TripwireHook(get_powered(nbt), get_attached(nbt), get_facing4(nbt)),
                "minecraft:redstone_torch" => Blocks::RedstoneTorch(Direction5::Down, get_lit(nbt)),
                "minecraft:redstone_wall_torch" => Blocks::RedstoneTorch(get_facing4(nbt).into(), get_lit(nbt)),
                "minecraft:hopper" => Blocks::Hopper(get_facing5(nbt), get_enabled(nbt)),
                "minecraft:redstone_lamp" => Blocks::RedstoneLamp(get_lit(nbt)),
                "minecraft:tnt" => Blocks::Tnt(get_unstable(nbt)),
                "minecraft:dispenser" => Blocks::Dispenser(get_facing6(nbt), get_triggered(nbt)),
                "minecraft:dropper" => Blocks::Dropper(get_facing6(nbt), get_triggered(nbt)),
                "minecraft:note_block" => Blocks::NoteBlock(get_powered(nbt), get_note(nbt), get_instrument(nbt)),
                "minecraft:repeater" => Blocks::Repeater(get_delay(nbt), get_facing4(nbt), get_locked(nbt), get_powered(nbt)),
                "minecraft:comparator" => Blocks::Comparator(get_facing4(nbt), get_comparator_mode(nbt), get_powered(nbt)),
                "minecraft:lectern" => Blocks::Lectern(get_facing4(nbt), get_powered(nbt), get_has_book(nbt)),
                "minecraft:piston" => Blocks::Piston(PistonType::Normal,get_facing6(nbt), get_extended(nbt)),
                "minecraft:sticky_piston" => Blocks::Piston(PistonType::Sticky,get_facing6(nbt), get_extended(nbt)),
                "minecraft:piston_head" => Blocks::PistonHead(get_piston_type(nbt), get_facing6(nbt), get_head_length(nbt)),
                "minecraft:target" => Blocks::Target(get_power(nbt)),
                "minecraft:lever" => Blocks::Lever(get_wall_face(nbt), get_facing4(nbt), get_powered(nbt)),
                "minecraft:heavy_weighted_pressure_plate" => Blocks::PressurePlate(PressurePlateType::Iron, get_powered_from_power(nbt)),
                "minecraft:light_weighted_pressure_plate" => Blocks::PressurePlate(PressurePlateType::Gold, get_powered_from_power(nbt)),
                "minecraft:stone_button" => Blocks::Button(ButtonType::Stone, get_powered(nbt)),
                "minecraft:polished_blackstone_button" => Blocks::Button(ButtonType::PolishedBlackstone, get_powered(nbt)),
                "minecraft:stone_pressure_plate" => Blocks::PressurePlate(PressurePlateType::Stone, get_powered(nbt)),
                "minecraft:polished_blackstone_pressure_plate" => Blocks::PressurePlate(PressurePlateType::PolishedBlackstone, get_powered(nbt)),
                "minecraft:lightning_rod" => Blocks::LightningRod(get_facing6(nbt), get_powered(nbt), get_waterlogged(nbt)),
                "minecraft:daylight_detector" => Blocks::DaylightDetector(get_inverted(nbt), get_power(nbt)),
                "minecraft:composter" => Blocks::Composter(get_level8(nbt)),
                "minecraft:stonecutter" => Blocks::StoneCutter(get_facing4(nbt)),
                "minecraft:carved_pumpkin" => Blocks::CarvedPumpkin(get_facing4(nbt)),
                "minecraft:jack_o_lantern" => Blocks::JackOLantern(get_facing4(nbt)),

                "minecraft:sea_pickle" => Blocks::SeaPickle(get_pickles(nbt), get_waterlogged(nbt)),
                "minecraft:kelp" => Blocks::Kelp(get_age25(nbt)), //ignore age
                "minecraft:chorus_flower" => Blocks::ChorusFlower(get_age5(nbt)),
                "minecraft:scaffolding" => Blocks::Scaffolding(get_distance(nbt), get_down(nbt), get_waterlogged(nbt)),
                "minecraft:campfire" => Blocks::Campfire(Soul::Normal,
                    get_facing4(nbt), get_signal(nbt), get_waterlogged(nbt), get_lit(nbt)),
                "minecraft:soul_campfire" => Blocks::Campfire(Soul::Soul,
                    get_facing4(nbt), get_signal(nbt), get_waterlogged(nbt), get_lit(nbt)),

                "minecraft:brown_mushroom_block" => {
                    Blocks::BrownMushroomBlock(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up),
                    get_side(nbt, Direction6::Down))
                }
                "minecraft:red_mushroom_block" => {
                    Blocks::RedMushroomBlock(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up),
                    get_side(nbt, Direction6::Down))
                }
                "minecraft:mushroom_stem" => {
                    Blocks::MushroomStem(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up),
                    get_side(nbt, Direction6::Down))
                }
                "minecraft:chorus_plant" => {
                    Blocks::ChorusPlant(get_side(nbt, Direction6::North),
                    get_side(nbt, Direction6::East),
                    get_side(nbt, Direction6::South),
                    get_side(nbt, Direction6::West),
                    get_side(nbt, Direction6::Up),
                    get_side(nbt, Direction6::Down))
                }
                "minecraft:chain" => Blocks::Chain(get_axis(nbt), get_waterlogged(nbt)),
                "minecraft:farmland" => Blocks::Farmland(get_moisture(nbt)),
                "minecraft:wheat" => Blocks::Crop7(Age7Crop::Wheat, get_age7(nbt)),
                "minecraft:carrots" => Blocks::Crop7(Age7Crop::Carrot, get_age7(nbt)),
                "minecraft:potatoes" => Blocks::Crop7(Age7Crop::Potato, get_age7(nbt)),
                "minecraft:melon_stem" => Blocks::Crop7(Age7Crop::MelonStem, get_age7(nbt)),
                "minecraft:pumpkin_stem" => Blocks::Crop7(Age7Crop::PumpkinStem, get_age7(nbt)),
                "minecraft:beetroots" => Blocks::Crop3(Age3Crop::Beetroot, get_age3(nbt)),
                "minecraft:nether_wart" => Blocks::Crop3(Age3Crop::NetherWart, get_age3(nbt)),
                "minecraft:attached_melon_stem" => Blocks::MelonStem(get_facing4(nbt)),
                "minecraft:attached_pumpkin_stem" => Blocks::PumpkinStem(get_facing4(nbt)),
                "minecraft:snow" => Blocks::Snow(get_layers8(nbt)),
                "minecraft:jukebox" => Blocks::Jukebox(get_record(nbt)),
                "minecraft:end_portal_frame" => Blocks::EndPortalFrame(get_eye(nbt), get_facing4(nbt)),
                "minecraft:bamboo" => Blocks::Bamboo(get_age1(nbt), get_leaves(nbt)), // ignore growth stage
                "minecraft:player_head" => Blocks::PlayerHead(get_rotation(nbt)),
                "minecraft:zombie_head" => Blocks::ZombieHead(get_rotation(nbt)),
                "minecraft:creeper_head" => Blocks::CreeperHead(get_rotation(nbt)),
                "minecraft:dragon_head" => Blocks::DragonHead(get_rotation(nbt)),
                "minecraft:skeleton_skull" => Blocks::SkeletonSkull(get_rotation(nbt)),
                "minecraft:wither_skeleton_skull" => Blocks::WitherSkeletonSkull(get_rotation(nbt)),
                "minecraft:loom" => Blocks::Loom(get_facing4(nbt)),
                "minecraft:barrel" => Blocks::Barrel(get_open(nbt), get_facing6(nbt)),
                "minecraft:respawn_anchor" => Blocks::RespawnAnchor(get_charges4(nbt)),
                "minecraft:lantern" => Blocks::Lantern(Soul::Normal, get_hanging(nbt), get_waterlogged(nbt)),
                "minecraft:soul_lantern" => Blocks::Lantern(Soul::Soul, get_hanging(nbt), get_waterlogged(nbt)),
                "minecraft:fire" => {
                    Blocks::Fire(Soul::Normal,
                        get_age15(nbt),
                        get_side(nbt, Direction6::North),
                        get_side(nbt, Direction6::East),
                        get_side(nbt, Direction6::South),
                        get_side(nbt, Direction6::West),
                        get_side(nbt, Direction6::Up))
                }
                "minecraft:soul_fire" => {
                    Blocks::Fire(Soul::Soul,
                        get_age15(nbt),
                        get_side(nbt, Direction6::North),
                        get_side(nbt, Direction6::East),
                        get_side(nbt, Direction6::South),
                        get_side(nbt, Direction6::West),
                        get_side(nbt, Direction6::Up))
                }
                "minecraft:brewing_stand" => {
                    Blocks::BrewingStand(get_bottle(nbt, 0), get_bottle(nbt, 1), get_bottle(nbt, 2))
                }
                "minecraft:turtle_egg" => Blocks::TurtleEgg(get_hatch2(nbt), get_eggs(nbt)),
                "minecraft:tripwire" => {
                    Blocks::Tripwire(
                        get_powered(nbt),
                        get_disarmed(nbt),
                        get_attached(nbt),
                        get_side(nbt, Direction6::North),
                        get_side(nbt, Direction6::East),
                        get_side(nbt, Direction6::South),
                        get_side(nbt, Direction6::West))
                }
                "minecraft:sweet_berry_bush" => Blocks::SweetBerryBush(get_age3(nbt)),

                "minecraft:waxed_oxidized_cut_copper_stairs" => Blocks::CutCopper(CopperType::Oxidized, Waxed::Waxed, get_stair(nbt)),
                "minecraft:waxed_weathered_cut_copper_stairs" => Blocks::CutCopper(CopperType::Weathered, Waxed::Waxed, get_stair(nbt)),
                "minecraft:waxed_exposed_cut_copper_stairs" => Blocks::CutCopper(CopperType::Exposed, Waxed::Waxed, get_stair(nbt)),
                "minecraft:waxed_cut_copper_stairs" => Blocks::CutCopper(CopperType::Copper, Waxed::Waxed, get_stair(nbt)),
                "minecraft:oxidized_cut_copper_stairs" => Blocks::CutCopper(CopperType::Oxidized, Waxed::Unwaxed, get_stair(nbt)),
                "minecraft:weathered_cut_copper_stairs" => Blocks::CutCopper(CopperType::Weathered, Waxed::Unwaxed, get_stair(nbt)),
                "minecraft:exposed_cut_copper_stairs" => Blocks::CutCopper(CopperType::Exposed, Waxed::Unwaxed, get_stair(nbt)),
                "minecraft:cut_copper_stairs" => Blocks::CutCopper(CopperType::Copper, Waxed::Unwaxed, get_stair(nbt)),
                "minecraft:waxed_oxidized_cut_copper_slab" => Blocks::CutCopper(CopperType::Oxidized, Waxed::Waxed, get_slab(nbt)),
                "minecraft:waxed_weathered_cut_copper_slab" => Blocks::CutCopper(CopperType::Weathered, Waxed::Waxed, get_slab(nbt)),
                "minecraft:waxed_exposed_cut_copper_slab" => Blocks::CutCopper(CopperType::Exposed, Waxed::Waxed, get_slab(nbt)),
                "minecraft:waxed_cut_copper_slab" => Blocks::CutCopper(CopperType::Copper, Waxed::Waxed, get_slab(nbt)),
                "minecraft:oxidized_cut_copper_slab" => Blocks::CutCopper(CopperType::Oxidized, Waxed::Unwaxed, get_slab(nbt)),
                "minecraft:weathered_cut_copper_slab" => Blocks::CutCopper(CopperType::Weathered, Waxed::Unwaxed, get_slab(nbt)),
                "minecraft:exposed_cut_copper_slab" => Blocks::CutCopper(CopperType::Exposed, Waxed::Unwaxed, get_slab(nbt)),
                "minecraft:cut_copper_slab" => Blocks::CutCopper(CopperType::Copper, Waxed::Unwaxed, get_slab(nbt)),

                "minecraft:glass_pane" => get_glass_pane(DyeColorClear::Clear, nbt),
                "minecraft:white_stained_glass_pane" => get_glass_pane(DyeColorClear::White, nbt),
                "minecraft:red_stained_glass_pane" => get_glass_pane(DyeColorClear::Red, nbt),
                "minecraft:orange_stained_glass_pane" => get_glass_pane(DyeColorClear::Orange, nbt),
                "minecraft:yellow_stained_glass_pane" => get_glass_pane(DyeColorClear::Yellow, nbt),
                "minecraft:green_stained_glass_pane" => get_glass_pane(DyeColorClear::Green, nbt),
                "minecraft:lime_stained_glass_pane" => get_glass_pane(DyeColorClear::Lime, nbt),
                "minecraft:blue_stained_glass_pane" => get_glass_pane(DyeColorClear::Blue, nbt),
                "minecraft:light_blue_stained_glass_pane" => get_glass_pane(DyeColorClear::LightBlue, nbt),
                "minecraft:cyan_stained_glass_pane" => get_glass_pane(DyeColorClear::Cyan, nbt),
                "minecraft:magenta_stained_glass_pane" => get_glass_pane(DyeColorClear::Magenta, nbt),
                "minecraft:pink_stained_glass_pane" => get_glass_pane(DyeColorClear::Pink, nbt),
                "minecraft:purple_stained_glass_pane" => get_glass_pane(DyeColorClear::Purple, nbt),
                "minecraft:brown_stained_glass_pane" => get_glass_pane(DyeColorClear::Brown, nbt),
                "minecraft:gray_stained_glass_pane" => get_glass_pane(DyeColorClear::Gray, nbt),
                "minecraft:light_gray_stained_glass_pane" => get_glass_pane(DyeColorClear::LightGray, nbt),
                "minecraft:black_stained_glass_pane" => get_glass_pane(DyeColorClear::Black, nbt),

                "minecraft:white_banner" => Blocks::Banner(DyeColor::White, get_rotation(nbt)),
                "minecraft:red_banner" => Blocks::Banner(DyeColor::Red, get_rotation(nbt)),
                "minecraft:orange_banner" => Blocks::Banner(DyeColor::Orange, get_rotation(nbt)),
                "minecraft:yellow_banner" => Blocks::Banner(DyeColor::Yellow, get_rotation(nbt)),
                "minecraft:green_banner" => Blocks::Banner(DyeColor::Green, get_rotation(nbt)),
                "minecraft:lime_banner" => Blocks::Banner(DyeColor::Lime, get_rotation(nbt)),
                "minecraft:blue_banner" => Blocks::Banner(DyeColor::Blue, get_rotation(nbt)),
                "minecraft:light_blue_banner" => Blocks::Banner(DyeColor::LightBlue, get_rotation(nbt)),
                "minecraft:cyan_banner" => Blocks::Banner(DyeColor::Cyan, get_rotation(nbt)),
                "minecraft:magenta_banner" => Blocks::Banner(DyeColor::Magenta, get_rotation(nbt)),
                "minecraft:pink_banner" => Blocks::Banner(DyeColor::Pink, get_rotation(nbt)),
                "minecraft:purple_banner" => Blocks::Banner(DyeColor::Purple, get_rotation(nbt)),
                "minecraft:brown_banner" => Blocks::Banner(DyeColor::Brown, get_rotation(nbt)),
                "minecraft:gray_banner" => Blocks::Banner(DyeColor::Gray, get_rotation(nbt)),
                "minecraft:light_gray_banner" => Blocks::Banner(DyeColor::LightGray, get_rotation(nbt)),
                "minecraft:black_banner" => Blocks::Banner(DyeColor::Black, get_rotation(nbt)),

                "minecraft:white_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::White, get_facing4(nbt)),
                "minecraft:red_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Red, get_facing4(nbt)),
                "minecraft:orange_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Orange, get_facing4(nbt)),
                "minecraft:yellow_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Yellow, get_facing4(nbt)),
                "minecraft:green_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Green, get_facing4(nbt)),
                "minecraft:lime_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Lime, get_facing4(nbt)),
                "minecraft:blue_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Blue, get_facing4(nbt)),
                "minecraft:light_blue_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::LightBlue, get_facing4(nbt)),
                "minecraft:cyan_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Cyan, get_facing4(nbt)),
                "minecraft:magenta_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Magenta, get_facing4(nbt)),
                "minecraft:pink_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Pink, get_facing4(nbt)),
                "minecraft:purple_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Purple, get_facing4(nbt)),
                "minecraft:brown_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Brown, get_facing4(nbt)),
                "minecraft:gray_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Gray, get_facing4(nbt)),
                "minecraft:light_gray_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::LightGray, get_facing4(nbt)),
                "minecraft:black_glazed_terracotta" => Blocks::GlazedTerracotta(DyeColor::Black, get_facing4(nbt)),

                "minecraft:shulker_box" => Blocks::ShulkerBox(DyeColorClear::Clear, get_facing6(nbt)),
                "minecraft:white_shulker_box" => Blocks::ShulkerBox(DyeColorClear::White, get_facing6(nbt)),
                "minecraft:red_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Red, get_facing6(nbt)),
                "minecraft:orange_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Orange, get_facing6(nbt)),
                "minecraft:yellow_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Yellow, get_facing6(nbt)),
                "minecraft:green_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Green, get_facing6(nbt)),
                "minecraft:lime_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Lime, get_facing6(nbt)),
                "minecraft:blue_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Blue, get_facing6(nbt)),
                "minecraft:light_blue_shulker_box" => Blocks::ShulkerBox(DyeColorClear::LightBlue, get_facing6(nbt)),
                "minecraft:cyan_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Cyan, get_facing6(nbt)),
                "minecraft:magenta_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Magenta, get_facing6(nbt)),
                "minecraft:pink_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Pink, get_facing6(nbt)),
                "minecraft:purple_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Purple, get_facing6(nbt)),
                "minecraft:brown_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Brown, get_facing6(nbt)),
                "minecraft:gray_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Gray, get_facing6(nbt)),
                "minecraft:light_gray_shulker_box" => Blocks::ShulkerBox(DyeColorClear::LightGray, get_facing6(nbt)),
                "minecraft:black_shulker_box" => Blocks::ShulkerBox(DyeColorClear::Black, get_facing6(nbt)),

                "minecraft:candle" => {
                    Blocks::Candle(DyeColorClear::Clear, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:white_candle" => {
                    Blocks::Candle(DyeColorClear::White, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:red_candle" => {
                    Blocks::Candle(DyeColorClear::Red, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:orange_candle" => {
                    Blocks::Candle(DyeColorClear::Orange, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:yellow_candle" => {
                    Blocks::Candle(DyeColorClear::Yellow, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:green_candle" => {
                    Blocks::Candle(DyeColorClear::Green, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:lime_candle" => {
                    Blocks::Candle(DyeColorClear::Lime, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:blue_candle" => {
                    Blocks::Candle(DyeColorClear::Blue, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:light_blue_candle" => {
                    Blocks::Candle(DyeColorClear::LightBlue, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:cyan_candle" => {
                    Blocks::Candle(DyeColorClear::Cyan, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:magenta_candle" => {
                    Blocks::Candle(DyeColorClear::Magenta, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:pink_candle" => {
                    Blocks::Candle(DyeColorClear::Pink, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:purple_candle" => {
                    Blocks::Candle(DyeColorClear::Purple, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:brown_candle" => {
                    Blocks::Candle(DyeColorClear::Brown, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:gray_candle" => {
                    Blocks::Candle(DyeColorClear::Gray, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:light_gray_candle" => {
                    Blocks::Candle(DyeColorClear::LightGray, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },
                "minecraft:black_candle" => {
                    Blocks::Candle(DyeColorClear::Black, get_lit(nbt), get_candles(nbt), get_waterlogged(nbt))
                },

                "minecraft:white_bed" => Blocks::Bed(DyeColor::White, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:red_bed" => Blocks::Bed(DyeColor::Red, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:orange_bed" => Blocks::Bed(DyeColor::Orange, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:yellow_bed" => Blocks::Bed(DyeColor::Yellow, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:green_bed" => Blocks::Bed(DyeColor::Green, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:lime_bed" => Blocks::Bed(DyeColor::Lime, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:blue_bed" => Blocks::Bed(DyeColor::Blue, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:light_blue_bed" => Blocks::Bed(DyeColor::LightBlue, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:cyan_bed" => Blocks::Bed(DyeColor::Cyan, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:magenta_bed" => Blocks::Bed(DyeColor::Magenta, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:pink_bed" => Blocks::Bed(DyeColor::Pink, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:purple_bed" => Blocks::Bed(DyeColor::Purple, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:brown_bed" => Blocks::Bed(DyeColor::Brown, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:gray_bed" => Blocks::Bed(DyeColor::Gray, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:light_gray_bed" => Blocks::Bed(DyeColor::LightGray, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                "minecraft:black_bed" => Blocks::Bed(DyeColor::Black, get_facing4(nbt), get_bed_end(nbt), get_occupied(nbt)),
                _ => panic!("{} {:?}", name, nbt),
            }
        }
    }
}

fn lvl_to_str<T: Into<Level25>>(lvl: T) -> &'static str {
    let lvl: Level25 = lvl.into();
    match lvl {
        Level25::Zero => "0",
        Level25::One => "1",
        Level25::Two => "2",
        Level25::Three => "3",
        Level25::Four => "4",
        Level25::Five => "5",
        Level25::Six => "6",
        Level25::Seven => "7",
        Level25::Eight => "8",
        Level25::Nine => "9",
        Level25::Ten => "10",
        Level25::Eleven => "11",
        Level25::Twelve => "12",
        Level25::Thirteen => "13",
        Level25::Fourteen => "14",
        Level25::Fifteen => "15",
        Level25::Sixteen => "16",
        Level25::Seventeen => "17",
        Level25::Eighteen => "18",
        Level25::Nineteen => "19",
        Level25::Twenty => "20",
        Level25::TwentyOne => "21",
        Level25::TwentyTwo => "22",
        Level25::TwentyThree => "23",
        Level25::TwentyFour => "24",
        Level25::TwentyFive => "25",
    }
}

fn dir_to_str<T: Into<Direction6> + Copy>(dir: T) -> &'static str {
    let dir: Direction6 = dir.into();
    match dir {
        Direction6::North => "north",
        Direction6::East => "east",
        Direction6::South => "south",
        Direction6::West => "west",
        Direction6::Down => "down",
        Direction6::Up => "up",
    }
}

fn slab_half_to_str<T: Into<SlabHalf> + Copy>(sh: T) -> &'static str {
    let sh: SlabHalf = sh.into();
    match sh {
        SlabHalf::Up => "top",
        SlabHalf::Down => "bottom",
        SlabHalf::Double => "double",
    }
}

fn stair_shape_to_str(w: StairShape) -> &'static str {
    match w {
        StairShape::InnerLeft => "inner_left",
        StairShape::InnerRight => "inner_right",
        StairShape::OuterLeft => "outer_left",
        StairShape::OuterRight => "outer_right",
        StairShape::Straight => "straight",
    }
}

fn water_logged_to_str(w: WaterLogged) -> &'static str {
    match w {
        WaterLogged::WaterLogged => "true",
        WaterLogged::Air => "false",
    }
}

fn side_to_str(w: Side) -> &'static str {
    match w {
        Side::True => "true",
        Side::False => "false",
    }
}

fn bloom_to_str(b: Bloom) -> &'static str {
    match b {
        Bloom::True => "true",
        Bloom::False => "false",
    }
}

fn chest_half_to_str(b: ChestHalf) -> &'static str {
    match b {
        ChestHalf::Left => "left",
        ChestHalf::Right => "right",
        ChestHalf::Single => "single",
    }
}

fn axis_to_str(a: Axis) -> &'static str {
    match a {
        Axis::X => "x",
        Axis::Y => "y",
        Axis::Z => "z",
    }
}

fn lit_to_str(a: Lit) -> &'static str {
    match a {
        Lit::Lit => "true",
        Lit::Unlit => "false",
    }
}

fn dye_to_str<T: Into<DyeColorClear>>(a: T) -> &'static str {
    let a: DyeColorClear = a.into();
    match a {
        DyeColorClear::White => "white",
        DyeColorClear::Red => "red",
        DyeColorClear::Orange => "orange",
        DyeColorClear::Yellow => "yellow",
        DyeColorClear::Green => "green",
        DyeColorClear::Lime => "lime",
        DyeColorClear::Blue => "blue",
        DyeColorClear::LightBlue => "light_blue",
        DyeColorClear::Cyan => "cyan",
        DyeColorClear::Magenta => "magenta",
        DyeColorClear::Pink => "pink",
        DyeColorClear::Purple => "purple",
        DyeColorClear::Brown => "brown",
        DyeColorClear::Gray => "gray",
        DyeColorClear::LightGray => "light_gray",
        DyeColorClear::Black => "black",
        DyeColorClear::Clear => "clear",
    }
}

fn wood_to_str<T: Into<DoorType>>(a: T) -> &'static str {
    let a: DoorType = a.into();
    match a {
        DoorType::Oak => "oak",
        DoorType::Birch => "birch",
        DoorType::Spruce => "spruce",
        DoorType::Jungle => "jungle",
        DoorType::DarkOak => "dark_oak",
        DoorType::Acacia => "acacia",
        DoorType::Crimson => "crimson",
        DoorType::Warped => "warped",
        DoorType::Mangrove => "mangrove",
        DoorType::Iron => "iron",
    }
}

fn dye_to_name<T: Into<DyeColorClear>>(a: T, part: &str, name: &str) -> String {
    let a: DyeColorClear = a.into();
    match a {
        DyeColorClear::Clear => name.to_string(),
        col => format!("{}{}{}", dye_to_str(col), part, name)
    }
}

fn block_nbt(name: &str) -> NBT {
    let mut map = HashMap::<String, NBT>::new();
    map.insert("name".to_string(), NBT::String(name.to_string()));
    NBT::Compound(map)
}

fn block_nbt_with_data(name: &str, nbt: NBT) -> NBT {
    let mut map = HashMap::<String, NBT>::new();
    map.insert("Name".to_string(), NBT::String(format!("minecraft:{name}")));
    map.insert("Properties".to_string(), nbt);
    NBT::Compound(map)
}

struct NBTMap(HashMap<String, NBT>);

impl NBTMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn to(self) -> NBT {
        NBT::Compound(self.0)
    }

    fn insert_level<T: Into<Level25> + Copy>(mut self, lvl: &T) -> Self {
        self.0.insert("level".to_string(), NBT::String(lvl_to_str(*lvl).to_string()));
        self
    }

    fn insert_age<T: Into<Level25> + Copy>(mut self, age: &T) -> Self {
        self.0.insert("age".to_string(), NBT::String(lvl_to_str(*age).to_string()));
        self
    }

    fn insert_stage<T: Into<Level25> + Copy>(mut self, stage: &T) -> Self {
        self.0.insert("stage".to_string(), NBT::String(lvl_to_str(*stage).to_string()));
        self
    }

    fn insert_facing<T: Into<Direction6> + Copy>(mut self, dir: &T) -> Self {
        self.0.insert("facing".to_string(), NBT::String(dir_to_str(*dir).to_string()));
        self
    }

    fn insert_snowy(mut self, snowy: Snowy) -> Self {
        self.0.insert("snowy".to_string(), NBT::String(match snowy {
            Snowy::True => "true",
            Snowy::False => "false",
        }.to_string()));
        self
    }

    fn insert_half(mut self, ud: UpDown) -> Self {
        self.0.insert("half".to_string(), NBT::String(slab_half_to_str(ud).to_string()));
        self
    }

    fn insert_slab_half(mut self, ud: SlabHalf) -> Self {
        self.0.insert("type".to_string(), NBT::String(slab_half_to_str(ud).to_string()));
        self
    }

    fn insert_shape(mut self, ud: StairShape) -> Self {
        self.0.insert("shape".to_string(), NBT::String(stair_shape_to_str(ud).to_string()));
        self
    }

    fn insert_side(mut self, side: &str, s: Side) -> Self {
        self.0.insert(side.to_string(), NBT::String(side_to_str(s).to_string()));
        self
    }

    fn insert_water_logged(mut self, w: WaterLogged) -> Self {
        self.0.insert("waterlogged".to_string(), NBT::String(water_logged_to_str(w).to_string()));
        self
    }

    fn insert_axis(mut self, a: Axis) -> Self {
        self.0.insert("axis".to_string(), NBT::String(axis_to_str(a).to_string()));
        self
    }

    fn insert_lit(mut self, a: Lit) -> Self {
        self.0.insert("lit".to_string(), NBT::String(lit_to_str(a).to_string()));
        self
    }

    fn insert_bloom(mut self, a: Bloom) -> Self {
        self.0.insert("bloom".to_string(), NBT::String(bloom_to_str(a).to_string()));
        self
    }

    fn insert_chest_half(mut self, a: ChestHalf) -> Self {
        self.0.insert("type".to_string(), NBT::String(chest_half_to_str(a).to_string()));
        self
    }
}

fn shaped_block_to_nbt(name: &str, shape: &ShapedBlock, suffix: &str) -> NBT {
    match shape {
        ShapedBlock::Stair(dir, side, shape, wlog) => {
            block_nbt_with_data(&format!("{}_stairs", name),
                NBTMap::new()
                    .insert_facing(dir)
                    .insert_half(*side)
                    .insert_shape(*shape)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Slab(side, wlog) => {
            block_nbt_with_data(&format!("{}_slab", name),
                NBTMap::new()
                    .insert_slab_half(*side)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Wall(n, e, s, w, u, wlog) => {
            block_nbt_with_data(&format!("{}_wall", name),
                NBTMap::new()
                    .insert_side("north", *n)
                    .insert_side("east", *e)
                    .insert_side("south", *s)
                    .insert_side("west", *w)
                    .insert_side("up", *u)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Fence(n, e, s, w, wlog) => {
            block_nbt_with_data(&format!("{}_fence", name),
                NBTMap::new()
                    .insert_side("north", *n)
                    .insert_side("east", *e)
                    .insert_side("south", *s)
                    .insert_side("west", *w)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Block => block_nbt(&format!("{name}{suffix}"))
    }
}

fn shaped_block_to_nbt_sb(name: &str, shape: &ShapedBlock, suffix: &str) -> NBT {
    match shape {
        ShapedBlock::Slab(side, wlog) => {
            block_nbt_with_data(&format!("{}_slab", name),
                NBTMap::new()
                    .insert_slab_half(*side)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Block => block_nbt(&format!("{name}{suffix}")),
        _ => panic!("not a minecraft block {name} : {shape:?}")
    }
}

fn shaped_block_to_nbt_ssb(name: &str, shape: &ShapedBlock, suffix: &str) -> NBT {
    match shape {
        ShapedBlock::Stair(dir, side, shape, wlog) => {
            block_nbt_with_data(&format!("{}_stairs", name),
                NBTMap::new()
                    .insert_facing(dir)
                    .insert_half(*side)
                    .insert_shape(*shape)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Slab(side, wlog) => {
            block_nbt_with_data(&format!("{}_slab", name),
                NBTMap::new()
                    .insert_slab_half(*side)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Block => block_nbt(&format!("{name}{suffix}")),
        _ => panic!("not a minecraft block {name} : {shape:?}")
    }
}

fn shaped_block_to_nbt_sswb(name: &str, shape: &ShapedBlock, suffix: &str) -> NBT {
    match shape {
        ShapedBlock::Stair(dir, side, shape, wlog) => {
            block_nbt_with_data(&format!("{}_stairs", name),
                NBTMap::new()
                    .insert_facing(dir)
                    .insert_half(*side)
                    .insert_shape(*shape)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Slab(side, wlog) => {
            block_nbt_with_data(&format!("{}_slab", name),
                NBTMap::new()
                    .insert_slab_half(*side)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Wall(n, e, s, w, u, wlog) => {
            block_nbt_with_data(&format!("{}_wall", name),
                NBTMap::new()
                    .insert_side("north", *n)
                    .insert_side("east", *e)
                    .insert_side("south", *s)
                    .insert_side("west", *w)
                    .insert_side("up", *u)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Block => block_nbt(&format!("{name}{suffix}")),
        _ => panic!("not a minecraft block {name} : {shape:?}")
    }
}

fn shaped_block_to_nbt_ssfb(name: &str, shape: &ShapedBlock, suffix: &str) -> NBT {
    match shape {
        ShapedBlock::Stair(dir, side, shape, wlog) => {
            block_nbt_with_data(&format!("{}_stairs", name),
                NBTMap::new()
                    .insert_facing(dir)
                    .insert_half(*side)
                    .insert_shape(*shape)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Slab(side, wlog) => {
            block_nbt_with_data(&format!("{}_slab", name),
                NBTMap::new()
                    .insert_slab_half(*side)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Fence(n, e, s, w, wlog) => {
            block_nbt_with_data(&format!("{}_wall", name),
                NBTMap::new()
                    .insert_side("north", *n)
                    .insert_side("east", *e)
                    .insert_side("south", *s)
                    .insert_side("west", *w)
                    .insert_water_logged(*wlog)
                    .to())
        },
        ShapedBlock::Block => block_nbt(&format!("{name}{suffix}")),
        _ => panic!("not a minecraft block {name} : {shape:?}")
    }
}

pub fn block_to_nbt(block: &Blocks) -> NBT {
    match block {
        Blocks::Crop7(crop, age) => {
            let age = NBTMap::new().insert_age(age).to();
            match crop {
                Age7Crop::Wheat => block_nbt_with_data("wheat", age),
                Age7Crop::Carrot => block_nbt_with_data("carrots", age),
                Age7Crop::Potato => block_nbt_with_data("potatoes", age),
                Age7Crop::MelonStem => block_nbt_with_data("melon_stem", age),
                Age7Crop::PumpkinStem => block_nbt_with_data("pumpkin_stem", age),
            }
        },
        Blocks::Crop3(crop, age) => {
            let age = NBTMap::new().insert_age(age).to();
            match crop {
                Age3Crop::Beetroot => block_nbt_with_data("beetroots", age),
                Age3Crop::NetherWart => block_nbt_with_data("nether_wart", age),
            }
        },
        Blocks::MelonStem(f) => block_nbt_with_data("attached_melon_stem",
            NBTMap::new().insert_facing(f).to()),
        Blocks::PumpkinStem(f) => block_nbt_with_data("attached_pumpkin_stem",
        NBTMap::new().insert_facing(f).to()),
        Blocks::Air => block_nbt("air"),
        Blocks::CaveAir => block_nbt("cave_air"),
        Blocks::Dirt => block_nbt("dirt"),
        Blocks::GrassBlock(t, s) => {
            let snowy = NBTMap::new().insert_snowy(*s).to();
            match t {
                GrassType::Grass => block_nbt_with_data("grass_block", snowy),
                GrassType::Mycelium => block_nbt_with_data("mycelium", snowy),
                GrassType::Podzol => block_nbt_with_data("podzol", snowy),
            }
        },
        Blocks::Grass => block_nbt("grass"),
        Blocks::Bedrock => block_nbt("bedrock"),
        Blocks::Stone(shape) => shaped_block_to_nbt_ssb("stone", shape, ""),
        Blocks::Cobblestone(shape) => shaped_block_to_nbt_sswb("cobblestone", shape, ""),
        Blocks::MossyCobblestone(shape) =>
            shaped_block_to_nbt_sswb("mossy_cobblestone", shape, ""),
        Blocks::Deepslate(a) => block_nbt_with_data("deepslate",
            NBTMap::new().insert_axis(*a).to()),
        Blocks::InfestedDeepslate(a) => block_nbt_with_data("infested_deepslate",
        NBTMap::new().insert_axis(*a).to()),
        Blocks::StoneBrick(shape) => shaped_block_to_nbt_sswb("stone_brick", shape, "s"),
        Blocks::MossyStoneBrick(shape) => shaped_block_to_nbt_sswb("mossy_stone_brick", shape, "s"),
        Blocks::SmoothStone(shape) => shaped_block_to_nbt_sb("smooth_stone", shape, ""),
        Blocks::CrackedStoneBricks => block_nbt("cracked_stone_bricks"),
        Blocks::CrackedDeepslateBricks => block_nbt("cracked_deepslate_bricks"),
        Blocks::DeepslateBricks(shape) => shaped_block_to_nbt_sswb("deepslate_brick", shape, "s"),
        Blocks::CobbledDeepslate(shape) => shaped_block_to_nbt_sswb("cobbled_deepslate", shape, ""),
        Blocks::ChiseledDeepslate => block_nbt("chiseled_deepslate"),
        Blocks::ChiseledStoneBrick => block_nbt("chiseled_stone_bricks"),
        Blocks::DeepslateTile(shape) => shaped_block_to_nbt_sswb("deepslate_tile", shape, "s"),
        Blocks::PolishedDeepslate(shape) => shaped_block_to_nbt_sswb("polished_deepslate", shape, ""),
        Blocks::Blackstone(shape) => shaped_block_to_nbt_sswb("blackstone", shape, ""),
        Blocks::PolishedBlackstone(shape) => shaped_block_to_nbt_sswb("polished_blackstone", shape, ""),
        Blocks::PolishedBlackstoneBrick(shape) => shaped_block_to_nbt_sswb("polished_blackstone_brick", shape, "s"),
        Blocks::Sandstone(col, shape) => {
            match col {
                Red::Normal => shaped_block_to_nbt_sswb("sandstone", shape, ""),
                Red::Red => shaped_block_to_nbt_sswb("red_sandstone", shape, ""),
            }
        },
        Blocks::Ore(ore) => {
            match ore {
                OreType::Copper => block_nbt("copper_ore"),
                OreType::Iron => block_nbt("iron_ore"),
                OreType::Gold => block_nbt("gold_ore"),
                OreType::Coal => block_nbt("coal_ore"),
                OreType::Lapis => block_nbt("lapis_ore"),
                OreType::Redstone(lit) => block_nbt_with_data("redstone_ore", NBTMap::new().insert_lit(*lit).to()),
                OreType::Diamond => block_nbt("diamond_ore"),
                OreType::Emerald => block_nbt("emerald_ore"),
            }
        },
        Blocks::DeepslateOre(ore) => {
            match ore {
                OreType::Copper => block_nbt("deepslate_copper_ore"),
                OreType::Iron => block_nbt("deepslate_iron_ore"),
                OreType::Gold => block_nbt("deepslate_gold_ore"),
                OreType::Coal => block_nbt("deepslate_coal_ore"),
                OreType::Lapis => block_nbt("lapis_ore"),
                OreType::Redstone(lit) => block_nbt_with_data("deepslate_redstone_ore", NBTMap::new().insert_lit(*lit).to()),
                OreType::Diamond => block_nbt("deepslate_diamond_ore"),
                OreType::Emerald => block_nbt("deepslate_emerald_ore"),
            }
        },
        Blocks::RawOre(ore) => {
            match ore {
                SmeltableOre::Copper => block_nbt("raw_copper_block"),
                SmeltableOre::Iron => block_nbt("raw_iron_block"),
                SmeltableOre::Gold => block_nbt("raw_gold_block"),
            }
        },
        Blocks::OreBlock(ore) => {
            match ore {
                OreType::Copper => block_nbt("copper_block"),
                OreType::Iron => block_nbt("iron_block"),
                OreType::Gold => block_nbt("gold_block"),
                OreType::Coal => block_nbt("coal_block"),
                OreType::Lapis => block_nbt("lapis_block"),
                OreType::Redstone(_) => block_nbt("redstone_block"),
                OreType::Diamond => block_nbt("diamond_block"),
                OreType::Emerald => block_nbt("emerald_block"),
            }
        },
        Blocks::Gravel => block_nbt("gravel"),
        Blocks::Tuff => block_nbt("tuff"),
        Blocks::Lava(lvl) => block_nbt_with_data("lava", NBTMap::new().insert_level(lvl).to()),
        Blocks::Water(lvl) => block_nbt_with_data("water", NBTMap::new().insert_level(lvl).to()),
        Blocks::GlowLichen(n, e, s, w, u, d, wat) =>
            block_nbt_with_data("glow_lichen",
                NBTMap::new()
                    .insert_side("north", *n)
                    .insert_side("east", *e)
                    .insert_side("south", *s)
                    .insert_side("west", *w)
                    .insert_side("up", *u)
                    .insert_side("down", *d)
                    .insert_water_logged(*wat).to()),
        Blocks::Vine(n, e, s, w, u) => 
            block_nbt_with_data("glow_lichen",
                NBTMap::new()
                    .insert_side("north", *n)
                    .insert_side("east", *e)
                    .insert_side("south", *s)
                    .insert_side("west", *w)
                    .insert_side("up", *u).to()),
        Blocks::Sculk => block_nbt("sculk"),
        Blocks::SculkVein(n, e, s, w, u, d, wat) => 
        block_nbt_with_data("sculk_vein",
            NBTMap::new()
                .insert_side("north", *n)
                .insert_side("east", *e)
                .insert_side("south", *s)
                .insert_side("west", *w)
                .insert_side("up", *u)
                .insert_side("down", *d)
                .insert_water_logged(*wat).to()),
        Blocks::SculkCatalist(bloom) => block_nbt_with_data("sculk_catalist", NBTMap::new().insert_bloom(*bloom).to()),
        Blocks::SculkSensor(level, wlog, phase) => todo!(),
        Blocks::SculkShrieker(can, wlog, shreik) => todo!(),
        Blocks::Chest(trap, half, dir, wlog) => {
            match trap {
                Trapped::True => 
                    block_nbt_with_data("trapped_chest",
                        NBTMap::new()
                        .insert_chest_half(*half)
                        .insert_facing(dir)
                        .insert_water_logged(*wlog)
                        .to()),
                Trapped::False => 
                    block_nbt_with_data("chest",
                        NBTMap::new()
                        .insert_chest_half(*half)
                        .insert_facing(dir)
                        .insert_water_logged(*wlog)
                        .to()),
            }
        },
        Blocks::EnderChest(dir, wlog) => 
            block_nbt_with_data("ender_chest",
                NBTMap::new()
                .insert_facing(dir)
                .insert_water_logged(*wlog)
                .to()),
        Blocks::Spawner => block_nbt("spawner"),
        Blocks::Andesite(shape) => shaped_block_to_nbt_sswb("andesite", shape, ""),
        Blocks::Granite(shape) => shaped_block_to_nbt_sswb("granite", shape, ""),
        Blocks::Diorite(shape) => shaped_block_to_nbt_sswb("diorite", shape, ""),
        Blocks::PolishedAndesite(shape) => shaped_block_to_nbt_ssb("polished_andesite", shape, ""),
        Blocks::PolishedGranite(shape) => shaped_block_to_nbt_ssb("polished_granite", shape, ""),
        Blocks::PolishedDiorite(shape) => shaped_block_to_nbt_ssb("polished_diorite", shape, ""),
        Blocks::InfestedStone => block_nbt("infested_stone"),
        Blocks::DripstoneBlock => block_nbt("dripstone_block"),
        Blocks::PointedDripstone(_, _, _) => todo!(),
        Blocks::Flower(_) => todo!(),
        Blocks::SmoothBasalt => block_nbt("smooth_basalt"),
        Blocks::Log(wood, typ, stripped, axis) => {
            let prefix = match stripped {
                Stripped::Stripped => "stripped_",
                Stripped::Unstripped => "",
            };
            let wood = wood_to_str(*wood);
            let suffix = match typ {
                LogType::Log => "_log",
                LogType::Wood => "_wood",
            };
            block_nbt_with_data(&format!("{prefix}{wood}{suffix}"),
                NBTMap::new()
                .insert_axis(*axis)
                .to())
        },
        Blocks::Sapling(w, stage) => {
            block_nbt_with_data(&format!("{}_sapling", wood_to_str(*w)),
                NBTMap::new().insert_stage(stage).to())
        },
        Blocks::Planks(w, shape) => {
            shaped_block_to_nbt_ssfb(wood_to_str(*w), shape, "_planks")
        },
        Blocks::Leaves(wood, dist, can, wlog) => todo!(),
        Blocks::PressurePlate(_, _) => todo!(),
        Blocks::Button(_, _) => todo!(),
        Blocks::Door(_, _, _, _, _, _) => todo!(),
        Blocks::Trapdoor(_, _, _, _, _, _) => todo!(),
        Blocks::FenceGate(_, _, _, _, _) => todo!(),
        Blocks::Sign(_, _, _) => todo!(),
        Blocks::MangrovePropagule(_, _, _, _) => todo!(),
        Blocks::MangroveRoots(_) => todo!(),
        Blocks::MuddyMangroveRoots(_) => todo!(),
        Blocks::HangingRoots(_) => todo!(),
        Blocks::Azalea => block_nbt("azalea"),
        Blocks::FloweringAzalea => block_nbt("flowering_azalea"),
        Blocks::WarpedRoots => block_nbt("warped_roots"),
        Blocks::SporeBlossom => block_nbt("spore_blossom"),
        Blocks::IronBars(_, _, _, _, _) => todo!(),
        Blocks::Calcite => block_nbt("calcite"),
        Blocks::AmethystBlock => block_nbt("amethyst_block"),
        Blocks::BuddingAmethyst => block_nbt("budding_amethyst"),
        Blocks::AmethystCluster(_, _, _) => todo!(),
        Blocks::DirtPath => block_nbt("dirt_path"),
        Blocks::HayBlock(_) => todo!(),
        Blocks::FlowerPot => block_nbt("flower_pot"),
        Blocks::Potted(_) => todo!(),
        Blocks::CrackedDeepslateTiles => block_nbt("cracked_deepslate_tiles"),
        Blocks::Bed(_, _, _, _) => todo!(),
        Blocks::Torch(_, _) => todo!(),
        Blocks::RedstoneTorch(_, _) => todo!(),
        Blocks::Furnace(_, _) => todo!(),
        Blocks::BlastFurnace(_, _) => todo!(),
        Blocks::Smoker(_, _) => todo!(),
        Blocks::PolishedBasalt(_) => todo!(),
        Blocks::WaterCauldron(_) => todo!(),
        Blocks::Grindstone(_, _) => todo!(),
        Blocks::Anvil(_, _) => todo!(),
        Blocks::Brick(shape) => shaped_block_to_nbt_sswb("brick", shape, "s"),
        Blocks::Sand(red) => {
            match red {
                Red::Normal => block_nbt("sand"),
                Red::Red => block_nbt("red_sand"),
            }
        },
        Blocks::Ladder(_, _) => todo!(),
        Blocks::Glass(dye) => block_nbt(&dye_to_name(*dye, "_stained_", "glass")),
        Blocks::GlassPane(_, _, _, _, _, _) => todo!(),
        Blocks::TintedGlass => block_nbt("tinted_glass"),
        Blocks::Candle(_, _, _, _) => todo!(),
        Blocks::Terracotta(dye) => block_nbt(&dye_to_name(*dye, "_", "terracotta")),
        Blocks::GlazedTerracotta(_, _) => todo!(),
        Blocks::Bell(_, _, _) => todo!(),
        Blocks::Wool(dye) => block_nbt(&dye_to_name(*dye, "_", "wool")),
        Blocks::Carpet(dye) => block_nbt(&dye_to_name(*dye, "_", "carpet")),
        Blocks::Seagrass => block_nbt("seagrass"),
        Blocks::TallFlower(_, _) => todo!(),
        Blocks::Clay => block_nbt("clay"),
        Blocks::MagmaBlock => block_nbt("magma_block"),
        Blocks::BubbleColumn(_) => todo!(),
        Blocks::SugarCane(_) => todo!(),
        Blocks::CaveVines(_, _) => todo!(),
        Blocks::CaveVinesPlant(_, _) => todo!(),
        Blocks::WeepingVines(_) => todo!(),
        Blocks::WeepingVinesPlant => block_nbt("weeping_vines_plant"),
        Blocks::TwistingVines(_) => todo!(),
        Blocks::TwistingVinesPlant => block_nbt("twisting_vines_plant"),
        Blocks::Obsidian => block_nbt("obsidian"),
        Blocks::Pumpkin => block_nbt("pumpkin"),
        Blocks::Melon => block_nbt("melon"),
        Blocks::BeeNest(_, _) => todo!(),
        Blocks::BeeHive(_, _) => todo!(),
        Blocks::MossBlock => block_nbt("moss_block"),
        Blocks::BigDripleaf(_, _, _) => todo!(),
        Blocks::BigDripleafStem(_, _) => todo!(),
        Blocks::SmallDripleaf(_, _, _) => todo!(),
        Blocks::Cobweb => block_nbt("cobweb"),
        Blocks::Rail(_, _) => todo!(),
        Blocks::ActivatorRail(_, _) => todo!(),
        Blocks::PoweredRail(_, _) => todo!(),
        Blocks::DetectorRail(_, _) => todo!(),
        Blocks::SeaPickle(_, _) => todo!(),
        Blocks::MossCarpet => block_nbt("moss_carpet"),
        Blocks::NetherSprouts => block_nbt("nether_sprouts"),
        Blocks::Kelp(_) => todo!(),
        Blocks::SlimeBlock => block_nbt("slime_block"),
        Blocks::HoneyBlock => block_nbt("honey_block"),
        Blocks::RedstoneWire(_, _, _, _, _) => todo!(),
        Blocks::Observer(_, _) => todo!(),
        Blocks::TripwireHook(_, _, _) => todo!(),
        Blocks::Hopper(_, _) => todo!(),
        Blocks::RedstoneLamp(_) => todo!(),
        Blocks::Tnt(unstable) => todo!(),
        Blocks::Dispenser(_, _) => todo!(),
        Blocks::Dropper(_, _) => todo!(),
        Blocks::NoteBlock(_, _, _) => todo!(),
        Blocks::Repeater(_, _, _, _) => todo!(),
        Blocks::Comparator(_, _, _) => todo!(),
        Blocks::Lectern(_, _, _) => todo!(),
        Blocks::Piston(_, _, _) => todo!(),
        Blocks::PistonHead(_, _, _) => todo!(),
        Blocks::Target(_) => todo!(),
        Blocks::Lever(_, _, _) => todo!(),
        Blocks::LightningRod(_, _, _) => todo!(),
        Blocks::DaylightDetector(_, _) => todo!(),
        Blocks::BrownMushroomBlock(_, _, _, _, _, _) => todo!(),
        Blocks::RedMushroomBlock(_, _, _, _, _, _) => todo!(),
        Blocks::MushroomStem(_, _, _, _, _, _) => todo!(),
        Blocks::Chain(_, _) => todo!(),
        Blocks::RootedDirt => block_nbt("rooted_dirt"),
        Blocks::Farmland(hydration) => todo!(),
        Blocks::SoulSand => block_nbt("soul_sand"),
        Blocks::CopperBlock(typ, waxed) => todo!(),
        Blocks::NetheriteBlock => block_nbt("netherite_block"),
        Blocks::CutCopper(typ, waxed, shape) => todo!(),
        Blocks::Composter(lvl) => todo!(),
        Blocks::StoneCutter(dir) => todo!(),
        Blocks::CrimsonNylium => block_nbt("crimson_nylium"),
        Blocks::WarpedNylium => block_nbt("warped_nylium"),
        Blocks::NetherGoldOre => block_nbt("nether_gold_ore"),
        Blocks::NetherQuartzOre => block_nbt("nether_quartz_ore"),
        Blocks::AncientDebris => block_nbt("ancient_debris"),
        Blocks::Sponge(wet) => {
            match wet {
                Wet::Wet => block_nbt("wet_sponge"),
                Wet::Dry => block_nbt("sponge"),
            }
        },
        Blocks::ChiseledSandstone(red) => {
            match red {
                Red::Normal => block_nbt("chiseled_sandstone"),
                Red::Red => block_nbt("chiseled_red_sandstone"),
            }
        },
        Blocks::CutSandstone(_, _) => todo!(),
        Blocks::CoarseDirt => block_nbt("coarse_dirt"),
        Blocks::Mud => block_nbt("mud"),
        Blocks::SmoothSandstone(red, shape) => {
            match red {
                Red::Normal => shaped_block_to_nbt_ssb("smooth_sandstone", shape, ""),
                Red::Red => shaped_block_to_nbt_ssb("smooth_red_sandstone", shape, ""),
            }
        },
        Blocks::SmoothQuartz(shape) => shaped_block_to_nbt_ssb("smooth_quartz", shape, ""),
        Blocks::Quartz(shape) => shaped_block_to_nbt_ssb("quartz", shape, ""),
        Blocks::CarvedPumpkin(dir) => 
            block_nbt_with_data("carved_pumpkin",
                NBTMap::new().insert_facing(dir).to()),
        Blocks::JackOLantern(dir) => 
            block_nbt_with_data("jack_o_lantern",
                NBTMap::new().insert_facing(dir).to()),
        Blocks::MudBrick(shape) => shaped_block_to_nbt_sswb("mud_brick", shape, "s"),
        Blocks::NetherBrick(shape) => shaped_block_to_nbt("netherbrick", shape, "s"),
        Blocks::RedNetherBrick(shape) => shaped_block_to_nbt_sswb("red_nether_brick", shape, "s"),
        Blocks::ChiseledNetherBrick => block_nbt("chiseled_nether_bricks"),
        Blocks::CrackedNetherBrick => block_nbt("cracked_nether_bricks"),
        Blocks::Purpur(shape) => shaped_block_to_nbt_ssb("purpur", shape, ""),
        Blocks::Prismarine(shape) => shaped_block_to_nbt_ssb("prismarine", shape, ""),
        Blocks::DarkPrismarine(shape) => shaped_block_to_nbt_ssb("dark_prismarine", shape, ""),
        Blocks::PrismarineBrick(shape) => shaped_block_to_nbt_ssb("prismarine_brick", shape, "s"),
        Blocks::Netherrack => block_nbt("netherrack"),
        Blocks::SnowBlock => block_nbt("snow_block"),
        Blocks::Ice(col) => {
            match col {
                IceType::Normal => block_nbt("ice"),
                IceType::Packed => block_nbt("packed_ice"),
                IceType::Blue => block_nbt("blue_ice"),
            }
        },
        Blocks::PurpurPillar(_) => todo!(),
        Blocks::Bookshelf => block_nbt("bookshelf"),
        Blocks::SoulSoil => block_nbt("soul_soil"),
        Blocks::Basalt(_) => todo!(),
        Blocks::Glowstone => block_nbt("glowstone"),
        Blocks::PackedMud => block_nbt("packed_mud"),
        Blocks::EndStoneBrick(_) => todo!(),
        Blocks::EndStone => block_nbt("end_stone"),
        Blocks::ReinforcedDeepslate => block_nbt("reinforced_deepslate"),
        Blocks::ChiseledQuartz => block_nbt("chiseled_quartz_block"),
        Blocks::QuartzBricks => block_nbt("quartz_bricks"),
        Blocks::QuartzPillar(_) => todo!(),
        Blocks::SeaLantern => block_nbt("sea_lantern"),
        Blocks::WarpedWartBlock => block_nbt("warped_wart_block"),
        Blocks::NetherWartBlock => block_nbt("nether_wart_block"),
        Blocks::BoneBlock(axis) => 
            block_nbt_with_data("bone_block",
                NBTMap::new().insert_axis(*axis).to()),
        Blocks::Concrete(dye) => block_nbt(&dye_to_name(*dye, "_", "concrete")),
        Blocks::ConcretePowder(dye) => block_nbt(&dye_to_name(*dye, "_", "concrete_powder")),
        Blocks::Coral(typ, dead, shape) => todo!(),
        Blocks::GildedBlackstone => block_nbt("gilded_blackstone"),
        Blocks::CryingObsidian => block_nbt("crying_obsidian"),
        Blocks::DriedKelpBlock => block_nbt("dried_kelp_block"),
        Blocks::ChiseledPolishedBlackstone => block_nbt("chiseled_polished_blackstone"),
        Blocks::CrackedPolishedBlackstoneBrick => block_nbt("cracked_polished_blackstone_bricks"),
        Blocks::ShulkerBox(_, _) => todo!(),
        Blocks::Snow(layers) => todo!(),
        Blocks::Jukebox(record) => todo!(),
        Blocks::InfestedCobblestone => block_nbt("infested_cobblestone"),
        Blocks::InfestedStoneBricks => block_nbt("infested_stone_bricks"),
        Blocks::InfestedCrackedStoneBricks => block_nbt("infested_cracked_stone_bricks"),
        Blocks::InfestedMossyStoneBricks => block_nbt("infested_mossy_stone_bricks"),
        Blocks::InfestedChiseledStoneBricks => block_nbt("infested_chiseled_stone_bricks"),
        Blocks::EndPortalFrame(_, _) => todo!(),
        Blocks::EnchantingTable => block_nbt("enchanting_table"),
        Blocks::CraftingTable => block_nbt("crafting_table"),
        Blocks::CartographyTable => block_nbt("cartography_table"),
        Blocks::FletchingTable => block_nbt("fletching_table"),
        Blocks::SmithingTable => block_nbt("smithing_table"),
        Blocks::ChorusFlower(_) => todo!(),
        Blocks::ChorusPlant(_, _, _, _, _, _) => todo!(),
        Blocks::Scaffolding(_, _, _) => todo!(),
        Blocks::Bamboo(_, _) => todo!(),
        Blocks::BambooSapling => block_nbt("bamboo_sapling"),
        Blocks::EndRod(dir) => 
            block_nbt_with_data("end_rod",
                NBTMap::new().insert_facing(dir).to()),
        Blocks::OchreFroglight(axis) => 
            block_nbt_with_data("ochre_froglight",
                NBTMap::new().insert_axis(*axis).to()),
        Blocks::VerdantFroglight(axis) => 
            block_nbt_with_data("verdant_froglight",
                NBTMap::new().insert_axis(*axis).to()),
        Blocks::PearlescentFroglight(axis) => 
            block_nbt_with_data("pearlescent_froglight",
                NBTMap::new().insert_axis(*axis).to()),
        Blocks::PlayerHead(_) => todo!(),
        Blocks::ZombieHead(_) => todo!(),
        Blocks::CreeperHead(_) => todo!(),
        Blocks::DragonHead(_) => todo!(),
        Blocks::SkeletonSkull(_) => todo!(),
        Blocks::WitherSkeletonSkull(_) => todo!(),
        Blocks::Banner(_, _) => todo!(),
        Blocks::Loom(dir) => 
            block_nbt_with_data("loom",
                NBTMap::new().insert_facing(dir).to()),
        Blocks::Barrel(_, _) => todo!(),
        Blocks::Campfire(_, _, _, _, _) => todo!(),
        Blocks::Shroomlight => block_nbt("shroomlite"),
        Blocks::RespawnAnchor(_) => todo!(),
        Blocks::Lodestone => block_nbt("lodestone"),
        Blocks::HoneycombBlock => block_nbt("honeycomb_block"),
        Blocks::Lantern(_, _, _) => todo!(),
        Blocks::LilyPad => block_nbt("lily_pad"),
        Blocks::Fire(_, _, _, _, _, _, _) => todo!(),
        Blocks::BrewingStand(_, _, _) => todo!(),
        Blocks::PowderSnow => block_nbt("powder_snow"),
        Blocks::TurtleEgg(_, _) => todo!(),
        Blocks::Beacon => block_nbt("beacon"),
        Blocks::Tripwire(_, _, _, _, _, _, _) => todo!(),
        Blocks::KelpPlant => block_nbt("kelp_plant"),
        Blocks::DragonEgg => block_nbt("dragon_egg"),
        Blocks::SweetBerryBush(_) => todo!(),
    }
}

pub fn palette_to_block(pal_nbt: &NBT) -> Result<Blocks, String> {
    let pal_map = pal_nbt.get_compound()?;
    // println!("pal_nbt {:?}", pal_nbt);
    let name = match pal_map.get("Name") {
        Some(v) => v.get_string()?,
        None => return Err("block has no name!".to_string()),
    };
    for (n, v) in pal_map.iter() {
        if n != "Name" && n != "Properties" {
            println!("unknown nbt tag in block {}: {} - {:?}", name, n, v);
        }
    }
    let nbt = match pal_map.get("Properties") {
        Some(NBT::Compound(t)) => Some(t),
        None => None,
        v => panic!("expected compound properties got {:?}", v)
    };
    Ok(string_to_block(name, nbt))
}
