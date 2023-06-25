using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using Saku.Models;

namespace Saku.Database.Configurations;

public class ChannelModelConfiguration : IEntityTypeConfiguration<ChannelModel>
{
    public void Configure(EntityTypeBuilder<ChannelModel> builder)
    {
        builder.HasKey(x => x.DiscordChannelId);

        builder.Property(x => x.State).HasConversion<ulong>().IsRequired();

        builder.Property(x => x.System).HasColumnType("text");

        builder.Property(x => x.Model).HasConversion<int>().IsRequired();

        builder.HasOne(x => x.BoundUser).WithMany(x => x.BoundedChannels).HasForeignKey(x => x.BoundUserId);
    }
}