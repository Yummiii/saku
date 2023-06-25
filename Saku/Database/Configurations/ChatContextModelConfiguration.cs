using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using Saku.Models;

namespace Saku.Database.Configurations;

public class ChatContextModelConfiguration : IEntityTypeConfiguration<ChatContextModel>
{
    public void Configure(EntityTypeBuilder<ChatContextModel> builder)
    {
        builder.HasKey(x => x.Id);

        builder.Property(x => x.ChatType).HasConversion<int>().IsRequired();

        builder.Property(x => x.Message).HasColumnType("text").IsRequired();

        builder.Property(x => x.IsPresentInCurrentContext).IsRequired();

        builder.Property(x => x.CreatedAt).IsRequired();

        builder.Property(x => x.ChannelId).IsRequired();
    }
}